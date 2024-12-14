use actix::Addr;
use ambassador::Delegate;
use anyhow::*;
use async_trait::async_trait;
use std::{result::Result::Ok, sync::Arc};

use super::event::MediaLibraryEventType;
use crate::{
    domain::task::task::{
        ambassador_impl_TaskIdentifiable, AsyncTask, TaskId, TaskIdentifiable, TaskIdentifier,
    },
    infrastructure::{
        event_bus::{domain_event::DomainEvent, event_bus::EventBus, model::GeneralEvent},
        organizer::organizer::{ParserActor, ScanMediaLibrary},
    },
};

#[derive(Delegate)]
#[delegate(TaskIdentifiable, target = "identifier")]
pub struct MediaLibraryScanTask {
    identifier: TaskIdentifier,
    library_path: String,
    parser_addr: Arc<Addr<ParserActor>>,
}

#[async_trait]
impl AsyncTask for MediaLibraryScanTask {
    async fn execute(&self, _identifier: TaskIdentifier, event_bus: Arc<EventBus>) -> Result<()> {
        let _ = event_bus.publish(DomainEvent::General(GeneralEvent::TaskStarted));

        let media_library = match self
            .parser_addr
            .send(ScanMediaLibrary(
                self.library_path.clone(),
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
        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

        let _ = event_bus.publish(DomainEvent::MediaLibrary(
            MediaLibraryEventType::MediaLibraryScanned {
                task_identifier: self.identifier.clone(),
                media_library,
            },
        ));

        Ok(())
    }
}

impl MediaLibraryScanTask {
    pub fn new(library_path: String, parser_addr: Arc<Addr<ParserActor>>) -> Self {
        Self {
            identifier: TaskIdentifier::default(),
            library_path,
            parser_addr,
        }
    }
}
