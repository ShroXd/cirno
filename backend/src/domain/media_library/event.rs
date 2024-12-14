use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::model::MediaLibrary;
use crate::domain::task::task::TaskIdentifier;

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
