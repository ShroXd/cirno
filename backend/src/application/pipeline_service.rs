use actix::{Actor, Addr};
use anyhow::*;
use std::{result::Result::Ok, sync::Arc, time::Duration};
use tokio::{spawn, sync::RwLock};
use tracing::*;

use crate::{
    domain::{
        pipeline::{
            builder::build_pipeline, event::PipelineEvent, model::Position, ports::PipelinePort,
            task::PipelinePreparationTask,
        },
        websocket::event::WebSocketEventType,
    },
    infrastructure::{
        event_bus::{domain_event::DomainEvent, event_bus::EventBus, handler::EventHandlerConfig},
        file::finder_options::{all_files, FinderOptions},
        hls::hls_state_actor::{HlsStateActor, SetPipelineAddr},
        pipeline::{actor::PipelineAction, pipeline::Pipeline},
        task_pool::{model::TaskType, task_pool::TaskPool},
    },
    interfaces::ws::utils::WsConnections,
    listen_event,
};

use super::file_service::FileService;

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

    #[instrument(skip(self, file_service, event_bus, task_pool))]
    pub async fn start_playback(
        &self,
        path: &str,
        file_service: Arc<FileService>,
        event_bus: Arc<EventBus>,
        task_pool: Arc<TaskPool>,
        ws_client_key: String,
        ws_connections: WsConnections,
    ) -> Result<String> {
        debug!("Deleting files in tmp folder");

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
            match build_pipeline(self.event_bus.clone(), self.hls_state_actor_addr.clone()) {
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

        Ok(task_id)
    }

    #[instrument(skip(self))]
    pub async fn seek_to_position(&self, position: Position) -> Result<()> {
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn stop_and_clean(&self) -> Result<()> {
        // TODO: consider if we need to move logic of stopping pipeline to service layer
        // TODO: once integrate with event bus and websocket, we also need to clean up them
        Ok(())
    }
}
