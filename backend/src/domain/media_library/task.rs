use actix::Addr;
use anyhow::*;
use async_trait::async_trait;
use std::{result::Result::Ok, sync::Arc};

use super::event::MediaLibraryEventType;
use crate::infrastructure::{
    event_bus::{
        event_bus::{DomainEvent, EventBus},
        model::GeneralEvent,
    },
    organizer::organizer::{ParserActor, ScanMediaLibrary},
    task_pool::model::AsyncTask,
};

pub struct MediaLibraryScanTask {
    library_path: String,
    parser_addr: Arc<Addr<ParserActor>>,
    task_id: String,
    ws_client_id: String,
}

#[async_trait]
impl AsyncTask for MediaLibraryScanTask {
    async fn execute(
        &self,
        ws_client_id: String,
        task_id: String,
        event_bus: Arc<EventBus>,
    ) -> Result<()> {
        let _ = event_bus.publish(
            DomainEvent::General(GeneralEvent::TaskStarted),
            self.task_id.clone(),
        );

        let media_library = match self
            .parser_addr
            .send(ScanMediaLibrary(
                self.library_path.clone(),
                self.task_id.clone(),
                event_bus.clone(),
            ))
            .await
            .map_err(|_| anyhow!("Failed to send scan media library message"))?
        {
            Ok(media_library) => media_library,
            Err(e) => return Err(anyhow!("Failed to scan media library: {:?}", e)),
        };

        // Artificial delay to test async task execution and UI feedback.
        // TODO: Remove this delay before production
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        let _ = event_bus.publish(
            DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibraryScanned(media_library)),
            self.task_id.clone(),
        );

        Ok(())
    }

    fn set_task_id(&mut self, task_id: String) {
        self.task_id = task_id;
    }

    fn get_task_id(&self) -> String {
        self.task_id.clone()
    }

    fn set_ws_client_id(&mut self, ws_client_id: String) {
        self.ws_client_id = ws_client_id;
    }

    fn get_ws_client_id(&self) -> String {
        self.ws_client_id.clone()
    }
}

impl MediaLibraryScanTask {
    pub fn new(library_path: String, parser_addr: Arc<Addr<ParserActor>>) -> Self {
        Self {
            library_path,
            parser_addr,
            task_id: String::new(),
            ws_client_id: String::new(),
        }
    }
}
