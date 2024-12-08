use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::model::MediaLibrary;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MediaLibraryEventType {
    MediaLibraryScanned(MediaLibrary),
    MediaLibrarySaved {
        media_library_id: i64,
        media_library_name: String,
    },
}
