use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use crate::domain::episode::model::Episode;

#[derive(Debug, Default, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Season {
    pub title: Option<String>,
    pub show_title: Option<String>,
    pub sort_title: Option<String>,
    pub year: Option<String>,
    pub plot: Option<String>,
    pub tvdb_id: Option<String>,
    pub imdb_id: Option<String>,
    pub tmdb_id: Option<String>,
    pub wikidata_id: Option<String>,
    pub premiered: Option<String>,

    // Information from the folder scanner
    pub season_number: Option<u8>,
    pub description: Option<String>,
    pub nfo_path: Option<String>,
    pub episodes: HashMap<u8, Episode>,
}
