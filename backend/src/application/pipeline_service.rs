use anyhow::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::*;

use crate::{
    domain::pipeline::{builder::build_pipeline, model::Position, ports::PipelinePort},
    infrastructure::event_bus::event_bus::EventBus,
};

#[derive(Clone)]
pub struct PipelineService {
    pipeline: Arc<Mutex<dyn PipelinePort>>,
}

impl PipelineService {
    #[instrument(skip(event_bus))]
    pub fn new(event_bus: Arc<EventBus>) -> Result<Self> {
        let pipeline = build_pipeline(event_bus)?;

        Ok(Self {
            pipeline: Arc::new(Mutex::new(pipeline)),
        })
    }

    #[instrument(skip(self))]
    pub async fn start_playback(&self, path: &str) -> Result<()> {
        debug!("Starting playback for path: {}", path);

        // TODO: Building a new pipeline for each source file may be inefficient.
        // Consider implementing a pipeline resource pool to reuse existing pipelines
        // instead of creating new ones each time. This would help with:
        // - Reducing memory usage and initialization overhead
        // - Better resource management
        // - Potentially faster playback starts
        self.pipeline.lock().await.build(path)?;
        self.pipeline.lock().await.play()?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn seek_to_position(&self, position: Position) -> Result<()> {
        let duration = self.pipeline.lock().await.get_duration()?;
        debug!("Duration: {:?}", duration);

        if position.as_nanos() > duration.as_nanos() {
            error!("Position is greater than the duration");
            return Err(anyhow!("Position is greater than the duration"));
        }

        debug!("Seeking to position: {:?}", position);
        self.pipeline.lock().await.seek(position)?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn stop_and_clean(&self) -> Result<()> {
        // TODO: consider if we need to move logic of stopping pipeline to service layer
        // TODO: once integrate with event bus and websocket, we also need to clean up them
        self.pipeline.lock().await.stop()?;
        Ok(())
    }
}
