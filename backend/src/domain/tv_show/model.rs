use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use crate::domain::{media_actor::model::MediaActor, season::model::Season};

#[derive(Debug, Default, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct TvShow {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub show_title: Option<String>,
    pub sort_title: Option<String>,
    pub year: Option<String>,
    pub plot: Option<String>,
    pub genres: Vec<String>,
    pub studios: Vec<String>,
    pub country: Option<String>,
    pub premiered: Option<String>,
    pub rating: Option<f32>,
    pub runtime: Option<String>,
    pub actors: Vec<MediaActor>,
    pub tmdb_id: Option<String>,
    pub imdb_id: Option<String>,
    pub wikidata_id: Option<String>,
    pub tvdb_id: Option<String>,

    // Information from the folder scanner
    pub nfo_path: Option<String>,
    pub poster_path: Option<String>,
    pub fanart_path: Option<String>,
    pub seasons: HashMap<u8, Season>,
}
