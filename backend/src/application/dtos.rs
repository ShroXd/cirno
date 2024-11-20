use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::http_api::controllers::api_models::MediaLibraryCategory;

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct MediaItemDto {
    pub id: i64,
    pub title: String,
    pub plot: Option<String>,
    pub poster_path: Option<String>,
    pub fanart_path: Option<String>,
    pub country: Option<String>,
    pub year: Option<String>,
    pub genres: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct MediaLibraryDto {
    pub id: i64,
    pub name: String,
    pub category: MediaLibraryCategory,
}
