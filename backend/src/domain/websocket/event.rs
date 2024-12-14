use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{define_payload, interfaces::ws::notification::ToJsonPayload};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum WebSocketEventType {
    RegisterClient(String), // client key
}

define_payload!(RegisterClient { client_key: String });

impl ToJsonPayload for WebSocketEventType {
    fn to_json_payload(&self) -> serde_json::Value {
        match self {
            WebSocketEventType::RegisterClient(client_key) => {
                serde_json::json!(RegisterClient::new(client_key.clone()))
            }
        }
    }
}
