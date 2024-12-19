use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use ts_rs::TS;

use crate::{
    domain::library::model::LibraryPoster, shared::util_traits::SqliteRowMapper,
};

use super::http_api::controllers::api_models::LibraryCategory;

#[derive(Debug, Deserialize, Serialize, TS, Clone)]
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

impl Default for MediaItemDto {
    fn default() -> Self {
        MediaItemDto {
            id: 0,
            title: String::new(),
            plot: None,
            poster_path: None,
            fanart_path: None,
            country: None,
            year: None,
            genres: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct LibraryDto {
    pub id: i64,
    pub name: String,
    pub category: LibraryCategory,
    pub posters: Vec<LibraryPoster>,
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
    pub episodes_number: Option<i64>,
    pub runtime: Option<i64>,
    pub season_number: Option<i64>,
    pub season_title: Option<String>,
}
