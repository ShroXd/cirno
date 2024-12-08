use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum NotificationType {
    RegisterClient,
    MediaLibrarySaved,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Notification<T> {
    pub event: NotificationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<T>,
}

pub trait IntoNotification {
    type Payload;
    fn into_notification(self) -> Notification<Self::Payload>;
}
