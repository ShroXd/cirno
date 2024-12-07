use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::model::MediaLibrary;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MediaLibraryEventType {
    MediaLibraryScanned(MediaLibrary),
    MediaLibrarySaved(i64), // media library id
}
