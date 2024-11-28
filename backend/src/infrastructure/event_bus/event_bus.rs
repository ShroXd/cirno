use anyhow::*;
use tokio::sync::broadcast;

use crate::domain::pipeline::events::PipelineEvent;

pub struct EventBus {
    sender: broadcast::Sender<PipelineEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<PipelineEvent> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: PipelineEvent) -> Result<()> {
        self.sender
            .send(event)
            .map_err(|_| anyhow!("Failed to send event"))?;

        Ok(())
    }
}
