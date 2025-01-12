use actix::{Actor, Addr};
use anyhow::*;
use std::{result::Result::Ok, sync::Arc};
use tokio::{spawn, sync::RwLock};
use tracing::*;

use super::file_service::FileService;
use crate::domain::task::task::TaskIdentifiable;
use crate::{
    domain::{
        pipeline::{
            builder::build_pipeline, event::PipelineEvent, model::Position,
            task::PipelinePreparationTask,
        },
        task::task::{TaskId, TaskType},
        websocket::event::WebSocketEventType,
    },
    infrastructure::{
        event_bus::{domain_event::DomainEvent, event_bus::EventBus, handler::EventHandlerConfig},
        file::finder_options::{all_files, FinderOptions},
        hls::hls_state_actor::{HlsStateActor, Reset, SetPipelineAddr},
        pipeline::actor::PipelineAction,
        task_pool::task_pool::TaskPool,
    },
    interfaces::ws::utils::WsConnections,
    listen_event,
};

#[derive(Clone)]
pub struct PipelineService {
    event_bus: Arc<EventBus>,
    hls_state_actor_addr: Addr<HlsStateActor>,
}

impl PipelineService {
    #[instrument(skip(event_bus))]
    pub fn new(
        event_bus: Arc<EventBus>,
        hls_state_actor_addr: Addr<HlsStateActor>,
    ) -> Result<Self> {
        let event_bus_clone = event_bus.clone();
        let ws_client_key = Arc::new(RwLock::new(Vec::new()));
        let ws_client_key_clone = ws_client_key.clone();

        spawn(async move {
            let mut subscription = event_bus_clone.subscribe();
            while let Ok(event) = subscription.recv().await {
                match event {
                    DomainEvent::WebSocket(WebSocketEventType::RegisterClient(key)) => {
                        debug!("Registering client to pipeline service with key: {}", key);
                        ws_client_key_clone.write().await.push(key);
                    }
                    _ => {}
                }
            }
        });

        Ok(Self {
            event_bus,
            hls_state_actor_addr,
        })
    }

    #[instrument(skip(self, file_service, task_pool))]
    pub async fn start_playback(
        &self,
        path: &str,
        file_service: Arc<FileService>,
        task_pool: Arc<TaskPool>,
        ws_client_key: String,
        ws_connections: WsConnections,
    ) -> Result<TaskId> {
        let event_bus = self.event_bus.clone();
        let mut task = PipelinePreparationTask::new(
            file_service.clone(),
            event_bus.clone(),
            self.hls_state_actor_addr.clone(),
        );
        task.set_ws_client_id(ws_client_key.clone());
        let task_id = task_pool
            .register_task(
                TaskType::PipelinePreparation,
                ws_client_key.clone(),
                Box::new(task),
                None,
            )
            .await?;

        let pipeline =
            match build_pipeline(path, event_bus.clone(), self.hls_state_actor_addr.clone()) {
                Ok(pipeline) => pipeline,
                Err(e) => return Err(anyhow::anyhow!("Failed to build pipeline: {}", e)),
            };

        let pipeline_addr = pipeline.start();
        self.hls_state_actor_addr
            .send(SetPipelineAddr(pipeline_addr.clone()))
            .await
            .inspect_err(|e| error!("Failed to set hls state actor pipeline address: {}", e))?;

        pipeline_addr
            .send(PipelineAction::Play)
            .await
            .inspect_err(|e| error!("Failed to start playback: {:?}", e))?;

        let ws_connection = match ws_connections.get(ws_client_key.clone()).await {
            Some(ws_connection) => ws_connection,
            None => {
                error!("WebSocket connection not found");
                return Err(anyhow!("WebSocket connection not found"));
            }
        };

        listen_event!(
            event_bus,
            DomainEvent::Pipeline(PipelineEvent::HlsStreamInitialized { .. }),
            move |event, _| {
                let ws_connection_clone = ws_connection.clone();
                async move {
                    if let Err(e) =
                        event.send_notification::<serde_json::Value>(ws_connection_clone)
                    {
                        error!("Failed to send hls stream initialized event: {:?}", e);
                    }
                    Ok(())
                }
            },
            EventHandlerConfig::one_time(),
        );

        let hls_state_actor_addr_clone = self.hls_state_actor_addr.clone();
        listen_event!(
            event_bus,
            DomainEvent::Pipeline(PipelineEvent::PipelineStopped),
            move |_, _| {
                let pipeline_addr_clone = pipeline_addr.clone();
                let hls_state_actor_addr_clone = hls_state_actor_addr_clone.clone();
                let file_service_clone = file_service.clone();

                async move {
                    let _ = pipeline_addr_clone
                        .send(PipelineAction::Stop)
                        .await
                        .map(|_| {
                            info!("Pipeline stopped");
                            Ok(())
                        })
                        .unwrap_or_else(|e| {
                            error!("Failed to stop pipeline: {:?}", e);
                            Err(anyhow::anyhow!("Failed to stop pipeline: {:?}", e))
                        });

                    let _ = hls_state_actor_addr_clone
                        .send(Reset)
                        .await
                        .inspect_err(|e| error!("Failed to reset hls state actor: {:?}", e));

                    let options = FinderOptions::new()
                        .filters(all_files())
                        .include_hidden(true);
                    let _ = file_service_clone
                        .delete_files_in_folder("./tmp", options)
                        .await
                        .inspect_err(|e| error!("Failed to delete files in tmp folder: {}", e));

                    Ok(())
                }
            },
            EventHandlerConfig::one_time(),
        );

        Ok(task_id)
    }

    #[instrument(skip(self))]
    pub async fn seek_to_position(&self, position: Position) -> Result<()> {
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn stop_and_clean(&self) -> Result<()> {
        match self
            .event_bus
            .publish(DomainEvent::Pipeline(PipelineEvent::PipelineStopped))
        {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!(
                "Failed to publish pipeline stopped event: {:?}",
                e
            )),
        }
    }
}
