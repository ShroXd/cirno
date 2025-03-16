use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::domain::{media_actor::model::MediaActor, media_library::model::LibraryPoster};

use super::http_api::controllers::api_models::LibraryCategory;

#[derive(Debug, Default, Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub struct MediaItemDto {
    pub id: i64,
    pub title: String,
    pub original_title: Option<String>,
    pub plot: Option<String>,
    pub poster_path: Option<String>,
    pub fanart_path: Option<String>,
    pub country: Option<String>,
    pub year: Option<String>,
    pub premiered: Option<String>,
    pub rating: Option<f64>,
    pub runtime: Option<i64>,
    pub genres: Vec<String>,
    pub studios: Vec<String>,
    pub actors: Vec<MediaActor>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, TS)]
#[ts(export)]
pub struct LibraryDto {
    pub id: i64,
    pub name: String,
    pub category: LibraryCategory,
    pub directory: String,
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
    pub episode_number: Option<i64>,
    pub runtime: Option<i64>,
    pub season_number: Option<i64>,
    pub season_title: Option<String>,
}
