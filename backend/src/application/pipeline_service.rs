use anyhow::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::*;

use crate::domain::pipeline::{model::Position, ports::PipelinePort};

pub struct PipelineService {
    pipeline: Arc<Mutex<dyn PipelinePort>>,
    // event_bus: Arc<EventBus>,
}

impl PipelineService {
    #[instrument(skip(pipeline))]
    pub fn new(pipeline: Arc<Mutex<dyn PipelinePort>>) -> Self {
        Self { pipeline }
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
}
