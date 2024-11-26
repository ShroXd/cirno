use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use crate::domain::{media_actor::model::Actor, season::model::Season};

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct TVSerie {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub show_title: Option<String>,
    pub sort_title: Option<String>,
    pub year: Option<String>,
    pub plot: Option<String>,
    pub genres: Vec<String>,
    pub country: Option<String>,
    pub actors: Vec<Actor>,
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
impl Default for TVSerie {
    fn default() -> Self {
        TVSerie {
            title: None,
            original_title: None,
            show_title: None,
            sort_title: None,
            year: None,
            plot: None,
            genres: Vec::new(),
            country: None,
            actors: Vec::new(),
            tmdb_id: None,
            imdb_id: None,
            wikidata_id: None,
            tvdb_id: None,
            nfo_path: None,
            poster_path: None,
            fanart_path: None,
            seasons: HashMap::new(),
        }
    }
}
