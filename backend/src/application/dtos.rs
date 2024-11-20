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

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct SeasonDto {
    pub season_number: Option<i64>,
    pub season_title: Option<String>,
    pub episodes: Vec<EpisodeDto>,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct EpisodeDto {
    pub id: i64,
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub plot: Option<String>,
    pub nfo_path: Option<String>,
    pub video_file_path: String,
    pub subtitle_file_path: Option<String>,
    pub thumb_image_url: Option<String>,
    pub thumb_image: Option<String>,
    pub season_number: Option<i64>,
    pub episodes_number: Option<i64>,
    pub runtime: Option<i64>,
}
