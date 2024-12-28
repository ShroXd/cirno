use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    domain::tv_show::model::TvShow, interfaces::http_api::controllers::api_models::LibraryCategory,
};

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Library {
    // TODO: maybe generic type
    pub tv_show: Vec<TvShow>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibraryBrief {
    pub id: i64,
    pub name: String,
    pub category: LibraryCategory,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct LibraryPoster {
    pub id: i64,
    // Base64 encoded image, would be changed in the future
    pub poster_path: Option<String>,
}
