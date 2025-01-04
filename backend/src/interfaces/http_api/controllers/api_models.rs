use serde::{Deserialize, Serialize};
use ts_rs::TS;

//------------------------------------------------------------------------------
// Create Media Library API Models
//------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct SaveLibraryPayload {
    pub name: String,
    pub directory: String,
    pub category: LibraryCategory,
}

#[derive(Debug, Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub struct UpdateLibraryPayload {
    #[ts(type = "number")]
    pub id: i64,
    pub name: String,
    pub directory: String,
    pub category: LibraryCategory,
}

#[derive(Debug, Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub enum LibraryCategory {
    Movie,
    TvShow,
    Animation,
}

impl From<LibraryCategory> for i64 {
    fn from(category: LibraryCategory) -> Self {
        match category {
            LibraryCategory::Movie => 1,
            LibraryCategory::TvShow => 2,
            LibraryCategory::Animation => 3,
        }
    }
}

impl TryFrom<i64> for LibraryCategory {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => LibraryCategory::Movie,
            2 => LibraryCategory::TvShow,
            3 => LibraryCategory::Animation,
            _ => return Err("Invalid library category".to_string()),
        })
    }
}

//------------------------------------------------------------------------------
// Get Media Items API Models
//------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct GetMediaItemsQuery {
    pub library_id: Option<i64>,
}

//------------------------------------------------------------------------------
// Video Player API Models
//------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct PlayVideoWithPathPayload {
    pub path: String,
}
