use actix::Addr;
use serde::Serialize;
use tracing::*;

use super::model::GeneralEvent;
use crate::{
    domain::{
        media_library::event::MediaLibraryEventType, pipeline::event::PipelineEvent,
        websocket::event::WebSocketEventType,
    },
    interfaces::ws::{
        actor::{SendNotification, WebSocketActor},
        notification::{IntoNotification, Notification, NotificationType},
    },
};

#[derive(Debug, Clone)]
pub enum DomainEvent {
    General(GeneralEvent),
    MediaLibrary(MediaLibraryEventType),
    Pipeline(PipelineEvent),
    WebSocket(WebSocketEventType),
}

impl IntoNotification for DomainEvent {
    type Payload = serde_json::Value;
    fn into_notification(self) -> Notification<Self::Payload> {
        match self {
            DomainEvent::MediaLibrary(event) => match event {
                MediaLibraryEventType::MediaLibrarySaved {
                    task_identifier,
                    media_library_id,
                    media_library_name,
                } => {
                    let payload = serde_json::json!({
                        "task_identifier": task_identifier,
                        "media_library_id": media_library_id,
                        "media_library_name": media_library_name,
                    });

                    Notification {
                        event: NotificationType::MediaLibrarySaved,
                        payload: Some(payload),
                    }
                }
                _ => unimplemented!(),
            },
            DomainEvent::WebSocket(payload) => Notification {
                event: NotificationType::RegisterClient,
                payload: Some(serde_json::to_value(payload).unwrap()),
            },
            DomainEvent::Pipeline(event) => match event {
                PipelineEvent::HlsStreamInitialized { path } => Notification {
                    event: NotificationType::HlsStreamInitialized,
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
