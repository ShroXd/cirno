use actix::Addr;
use serde::Serialize;
use tracing::*;

use super::model::GeneralEvent;
use crate::{
    domain::{
        media_library::event::MediaLibraryEventType, pipeline::event::PipelineEvent,
        websocket::event::WebSocketEvent,
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
    WebSocket(WebSocketEvent),
}

impl IntoNotification for DomainEvent {
    type Payload = serde_json::Value;
    fn into_notification(self) -> Notification<Self::Payload> {
        match self {
            DomainEvent::MediaLibrary(payload) => Notification {
                event: NotificationType::MediaLibraryScanned,
                payload: Some(serde_json::to_value(payload).unwrap()),
            },
            DomainEvent::WebSocket(payload) => Notification {
                event: NotificationType::RegisterClient,
                payload: Some(serde_json::to_value(payload).unwrap()),
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
