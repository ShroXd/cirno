use serde::{Deserialize, Serialize};
use ts_rs::TS;

//------------------------------------------------------------------------------
// Create Media Library API Models
//------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct SaveMediaLibraryPayload {
    pub name: String,
    pub directory: String,
    pub category: MediaLibraryCategory,
}

#[derive(Debug, Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub enum MediaLibraryCategory {
    Movie,
    TvShow,
    Animation,
}

impl From<MediaLibraryCategory> for i64 {
    fn from(category: MediaLibraryCategory) -> Self {
        match category {
            MediaLibraryCategory::Movie => 1,
            MediaLibraryCategory::TvShow => 2,
            MediaLibraryCategory::Animation => 3,
        }
    }
}

impl TryFrom<i64> for MediaLibraryCategory {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => MediaLibraryCategory::Movie,
            2 => MediaLibraryCategory::TvShow,
            3 => MediaLibraryCategory::Animation,
            _ => return Err("Invalid media library category".to_string()),
        })
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct CreateMediaLibraryResponse {
    pub media_library_id: i64,
    pub async_task_id: String,
}

//------------------------------------------------------------------------------
// Get Media Items API Models
//------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct GetMediaItemsQuery {
    pub media_library_id: Option<i64>,
}
