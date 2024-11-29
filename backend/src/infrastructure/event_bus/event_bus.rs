use anyhow::*;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::domain::pipeline::events::PipelineEvent;

#[derive(Debug, Clone)]
pub enum OtherEventType {
    StreamError(String),
    MediaLibraryScanned(String), // ws client id
    TaskProgressUpdated(f32),    // progress
}

#[derive(Debug, Clone)]
pub enum EventType {
    Pipeline(PipelineEvent),
    Other(OtherEventType),
}

#[derive(Debug, Clone)]
pub struct EventBus {
    tx: broadcast::Sender<(EventType, String)>, // (event, task id)
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<(EventType, String)> {
        self.tx.subscribe()
    }

    pub fn publish(&self, event: EventType, task_id: String) -> Result<()> {
        self.tx
            .send((event, task_id))
            .map_err(|_| anyhow!("Failed to send event"))?;

        Ok(())
    }
}
