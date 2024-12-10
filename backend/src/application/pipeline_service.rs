use actix::{spawn, Actor, Addr};
use anyhow::*;
use std::{result::Result::Ok, sync::Arc};
use tokio::sync::RwLock;
use tracing::*;

use crate::{
    domain::{
        pipeline::{builder::build_pipeline, model::Position, ports::PipelinePort},
        websocket::event::WebSocketEventType,
    },
    infrastructure::{
        event_bus::{domain_event::DomainEvent, event_bus::EventBus},
        hls::hls_state_actor::{HlsStateActor, SetPipelineAddr},
        pipeline::{actor::PipelineAction, pipeline::Pipeline},
    },
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

    #[instrument(skip(self))]
    pub async fn start_playback(&self, path: &str) -> Result<()> {
        debug!("Starting playback for path: {}", path);

        let pipeline =
            match build_pipeline(self.event_bus.clone(), self.hls_state_actor_addr.clone()) {
                Ok(pipeline) => pipeline,
                Err(e) => return Err(anyhow::anyhow!("Failed to build pipeline: {}", e)),
            };

        let pipeline_addr = pipeline.start();
        match self
            .hls_state_actor_addr
            .send(SetPipelineAddr(pipeline_addr.clone()))
            .await
        {
            Ok(_) => info!("Hls state actor set pipeline address"),
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to set hls state actor pipeline address: {}",
                    e
                ))
            }
        }

        if let Err(e) = pipeline_addr.send(PipelineAction::Play).await {
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
