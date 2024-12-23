use actix::{Actor, Addr};
use anyhow::*;
use std::{result::Result::Ok, sync::Arc};
use tokio::{spawn, sync::RwLock};
use tracing::*;

use super::file_service::FileService;
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
        hls::hls_state_actor::{HlsStateActor, SetPipelineAddr},
        pipeline::actor::PipelineAction,
        task_pool::task_pool::TaskPool,
    },
    interfaces::ws::utils::WsConnections,
    listen_event,
};

#[derive(Clone)]
pub struct PipelineService {
    event_bus: Arc<EventBus>,
    ws_client_key: Arc<RwLock<Vec<String>>>,
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
            ws_client_key,
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
        let task = PipelinePreparationTask::new(
            file_service,
            event_bus.clone(),
            self.hls_state_actor_addr.clone(),
        );
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
                    event.send_notification::<serde_json::Value>(ws_connection_clone);
                    Ok(())
                }
            },
            EventHandlerConfig::one_time(),
        );

        listen_event!(
            event_bus,
            DomainEvent::Pipeline(PipelineEvent::PipelineStopped),
            move |_, _| {
                let pipeline_addr_clone = pipeline_addr.clone();
                async move {
                    match pipeline_addr_clone.send(PipelineAction::Stop).await {
                        Ok(_) => {
                            info!("Pipeline stopped");
                            Ok(())
                        }
                        Err(e) => {
                            error!("Failed to stop pipeline: {:?}", e);
                            Err(anyhow::anyhow!("Failed to stop pipeline: {:?}", e))
                        }
                    }
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
