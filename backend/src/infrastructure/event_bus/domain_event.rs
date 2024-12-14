use actix::Addr;
use serde::Serialize;
use tracing::*;

use super::model::GeneralEvent;
use crate::{
    domain::{
        media_library::event::MediaLibraryEventType, pipeline::event::PipelineEvent,
        task::task::AsyncTaskEvent, websocket::event::WebSocketEventType,
    },
    interfaces::ws::{
        actor::{SendNotification, WebSocketActor},
        notification::{IntoNotification, Notification, NotificationType, ToJsonPayload},
    },
};

#[derive(Debug, Clone, Serialize)]
pub enum DomainEvent {
    General(GeneralEvent),
    AsyncTask(AsyncTaskEvent),
    MediaLibrary(MediaLibraryEventType),
    Pipeline(PipelineEvent),
    WebSocket(WebSocketEventType),
}

impl IntoNotification for DomainEvent {
    type Payload = serde_json::Value;
    fn into_notification(self) -> Notification<Self::Payload> {
        match self {
            DomainEvent::AsyncTask(event) => match event {
                AsyncTaskEvent::ProgressUpdated { .. } => {
                    Notification::new(NotificationType::TaskProgressUpdated, event)
                }
                _ => unimplemented!(),
            },
            DomainEvent::MediaLibrary(event) => match event {
                MediaLibraryEventType::MediaLibrarySaved { .. } => {
                    Notification::new(NotificationType::MediaLibrarySaved, event)
                }
                _ => unimplemented!(),
            },
            DomainEvent::Pipeline(event) => match event {
                PipelineEvent::HlsStreamInitialized { .. } => {
                    Notification::new(NotificationType::HlsStreamInitialized, event)
                }
                _ => unimplemented!(),
            },
            DomainEvent::WebSocket(event) => {
                Notification::new(NotificationType::RegisterClient, event)
            }
            _ => unimplemented!(),
        }
    }
}

impl DomainEvent {
    #[instrument(skip(self, addr))]
    pub fn send_notification<T>(self, addr: Addr<WebSocketActor>)
    where
        T: Serialize,
    {
        let notification = self.into_notification();

        debug!("Sending notification: {:?}", notification);
        match addr.try_send(SendNotification(notification)) {
            Ok(_) => (),
            Err(e) => error!("Failed to send notification: {:?}", e),
        }
    }
}
