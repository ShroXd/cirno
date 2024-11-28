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
    pub async fn start_playback(&self, uri: &str) -> Result<()> {
        debug!("Starting playback for URI: {}", uri);

        self.pipeline.lock().await.build()?;
        self.pipeline.lock().await.play()?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn seek_to_position(&self, position: u64) -> Result<()> {
        let position = Position::from_secs(position)?;
        debug!("Position: {:?}", position);

        let duration = self.pipeline.lock().await.get_duration()?;
        debug!("Duration: {:?}", duration);

        if position.as_nanos() > duration.as_nanos() {
            error!("Position is greater than the duration");
            return Err(anyhow!("Position is greater than the duration"));
        }

        debug!("Seeking to position: {:?}", position);
        self.pipeline
            .lock()
            .await
            .seek(position.as_nanos() as u32)?;

        Ok(())
    }
}
