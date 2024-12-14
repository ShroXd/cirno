use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::model::MediaLibrary;
use crate::{
    define_payload, domain::task::task::TaskIdentifier, interfaces::ws::notification::ToJsonPayload,
};

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

define_payload!(MediaLibraryScanned {
    task_identifier: TaskIdentifier,
    media_library: MediaLibrary,
});

define_payload!(MediaLibrarySaved {
    task_identifier: TaskIdentifier,
    media_library_id: i64,
    media_library_name: String,
});

impl ToJsonPayload for MediaLibraryEventType {
    fn to_json_payload(&self) -> serde_json::Value {
        match self {
            MediaLibraryEventType::MediaLibraryScanned {
                task_identifier,
                media_library,
            } => {
                serde_json::json!(MediaLibraryScanned::new(
                    task_identifier.to_owned(),
                    media_library.to_owned()
                ))
            }
            MediaLibraryEventType::MediaLibrarySaved {
                task_identifier,
                media_library_id,
                media_library_name,
            } => {
                serde_json::json!(MediaLibrarySaved::new(
                    task_identifier.to_owned(),
                    media_library_id.to_owned(),
                    media_library_name.to_owned()
                ))
            }
        }
    }
}
