use actix::Addr;
use anyhow::*;
use serde::Serialize;
use std::result::Result::Ok;
use tracing::*;

use super::model::GeneralEvent;
use crate::{
    domain::{
        library::event::LibraryEventType, pipeline::event::PipelineEvent,
        task::task::AsyncTaskEvent, websocket::event::WebSocketEventType,
    },
    interfaces::ws::{
        actor::{SendNotification, WebSocketActor},
        notification::{IntoNotification, Notification, NotificationType},
    },
};

#[derive(Debug, Clone, Serialize)]
pub enum DomainEvent {
    General(GeneralEvent),
    AsyncTask(AsyncTaskEvent),
    Library(LibraryEventType),
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
            DomainEvent::Library(event) => match event {
                LibraryEventType::LibrarySaved { .. } => {
                    Notification::new(NotificationType::LibrarySaved, event)
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
    pub fn send_notification<T>(self, addr: Addr<WebSocketActor>) -> Result<()>
    where
        T: Serialize,
    {
        let notification = self.into_notification();

        debug!("Sending notification: {:?}", notification);
        match addr.try_send(SendNotification(notification)) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Failed to send notification: {:?}", e)),
        }
    }
}
