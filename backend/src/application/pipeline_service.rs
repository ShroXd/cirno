use actix::{spawn, Addr};
use anyhow::*;
use std::{result::Result::Ok, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tracing::*;

use crate::{
    domain::{
        pipeline::{builder::build_pipeline, model::Position, ports::PipelinePort},
        websocket::event::WebSocketEvent,
    },
    infrastructure::{
        event_bus::{domain_event::DomainEvent, event_bus::EventBus},
        pipeline::{actor::PipelineAction, pipeline::Pipeline},
    },
};

#[derive(Clone)]
pub struct PipelineService {
    pipeline_addr: Addr<Pipeline>,
    event_bus: Arc<EventBus>,
    ws_client_key: Arc<RwLock<Vec<String>>>,
}

impl PipelineService {
    #[instrument(skip(pipeline_addr, event_bus))]
    pub fn new(pipeline_addr: Addr<Pipeline>, event_bus: Arc<EventBus>) -> Result<Self> {
        let event_bus_clone = event_bus.clone();
        let ws_client_key = Arc::new(RwLock::new(Vec::new()));
        let ws_client_key_clone = ws_client_key.clone();

        spawn(async move {
            let mut subscription = event_bus_clone.subscribe();
            while let Ok(event) = subscription.recv().await {
                match event {
                    (DomainEvent::WebSocket(WebSocketEvent::RegisterClient { key }), _) => {
                        debug!("Registering client to pipeline service with key: {}", key);
                        ws_client_key_clone.write().await.push(key);
                    }
                    _ => {}
                }
            }
        });

        Ok(Self {
            pipeline_addr,
            event_bus,
            ws_client_key,
        })
    }

    #[instrument(skip(self))]
    pub async fn start_playback(&self, path: &str) -> Result<()> {
        debug!("Starting playback for path: {}", path);

        if let Err(e) = self
            .pipeline_addr
            .send(PipelineAction::SetSource(path.to_string()))
            .await
        {
            error!("Failed to set source: {:?}", e);
            return Err(anyhow!("Failed to set source"));
        }

        if let Err(e) = self.pipeline_addr.send(PipelineAction::Play).await {
            error!("Failed to start playback: {:?}", e);
            return Err(anyhow!("Failed to start playback"));
        }

        Ok(())
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
