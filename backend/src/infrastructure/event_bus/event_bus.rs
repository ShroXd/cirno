use anyhow::*;
use tokio::sync::broadcast;

use super::model::GeneralEvent;
use crate::domain::{media_library::event::MediaLibraryEventType, pipeline::event::PipelineEvent};

#[derive(Debug, Clone)]
pub enum DomainEvent {
    General(GeneralEvent),
    MediaLibrary(MediaLibraryEventType),
    Pipeline(PipelineEvent),
}

#[derive(Debug, Clone)]
pub struct EventBus {
    tx: broadcast::Sender<(DomainEvent, String)>, // (event, task id)
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<(DomainEvent, String)> {
        self.tx.subscribe()
    }

    pub fn publish(&self, event: DomainEvent, task_id: String) -> Result<()> {
        self.tx
            .send((event, task_id))
            .map_err(|_| anyhow!("Failed to send event"))?;

        Ok(())
    }
}
