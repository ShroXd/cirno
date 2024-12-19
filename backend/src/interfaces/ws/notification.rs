use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum NotificationType {
    RegisterClient,
    LibrarySaved,
    HlsStreamInitialized,
    TaskProgressUpdated,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Notification<T> {
    pub notification_type: NotificationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<T>,
}

impl Notification<serde_json::Value> {
    pub fn new(notification_type: NotificationType, payload: impl ToJsonPayload) -> Self {
        Self {
            notification_type,
            payload: Some(payload.to_json_payload()),
        }
    }
}

pub trait IntoNotification {
    type Payload;
    fn into_notification(self) -> Notification<Self::Payload>;
}

pub trait ToJsonPayload {
    fn to_json_payload(&self) -> serde_json::Value;
}
