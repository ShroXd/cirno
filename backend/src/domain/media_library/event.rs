use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::model::MediaLibrary;
use crate::{domain::task::task::TaskIdentifier, interfaces::ws::notification::ToJsonPayload};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MediaLibraryEventType {
    MediaLibraryScanned {
        task_identifier: TaskIdentifier,
        media_library: MediaLibrary,
    },
    MediaLibrarySaved {
        task_identifier: TaskIdentifier,
        media_library_id: i64,
        media_library_name: String,
    },
}

impl ToJsonPayload for MediaLibraryEventType {
    fn to_json_payload(&self) -> serde_json::Value {
        match self {
            MediaLibraryEventType::MediaLibraryScanned {
                task_identifier,
                media_library,
            } => {
                serde_json::json!({
                    "type": "MediaLibraryScanned",
                    "task_identifier": task_identifier,
                    "media_library": media_library
                })
            }
            MediaLibraryEventType::MediaLibrarySaved {
                task_identifier,
                media_library_id,
                media_library_name,
            } => {
                serde_json::json!({
                    "type": "MediaLibrarySaved",
                    "task_identifier": task_identifier,
                    "media_library_id": media_library_id,
                    "media_library_name": media_library_name
                })
            }
        }
    }
}
