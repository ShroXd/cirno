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
                AsyncTaskEvent::ProgressUpdated { .. } => Notification {
                    notification_type: NotificationType::TaskProgressUpdated,
                    payload: Some(event.to_json_payload()),
                },
                _ => unimplemented!(),
            },
            DomainEvent::MediaLibrary(event) => match event {
                MediaLibraryEventType::MediaLibrarySaved { .. } => Notification {
                    notification_type: NotificationType::MediaLibrarySaved,
                    payload: Some(event.to_json_payload()),
                },
                _ => unimplemented!(),
            },
            DomainEvent::WebSocket(payload) => Notification {
                notification_type: NotificationType::RegisterClient,
                payload: Some(serde_json::to_value(payload).unwrap()),
            },
            DomainEvent::Pipeline(event) => match event {
                PipelineEvent::HlsStreamInitialized { path } => Notification {
                    notification_type: NotificationType::HlsStreamInitialized,
                    payload: Some(serde_json::to_value(path).unwrap()),
                },
                _ => unimplemented!(),
            },
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
