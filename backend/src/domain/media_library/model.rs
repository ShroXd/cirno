use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    domain::{library::model::LibraryStatus, tv_show::model::TvShow},
    interfaces::http_api::controllers::api_models::LibraryCategory,
};

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Library {
    // TODO: maybe generic type
    pub tv_show: Vec<TvShow>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct LibraryBrief {
    pub id: i64,
    pub name: String,
    pub category: LibraryCategory,
    pub directory: String,
    pub item_count: i64,
    pub last_scanned: Option<String>,
    pub current_status: LibraryStatus,
    pub auto_scan: bool,
    pub error: Option<String>,
    pub storage_used: i64,
    pub health_score: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, TS, Default)]
#[ts(export)]
pub struct LibraryPoster {
    pub id: i64,
    // Base64 encoded image, would be changed in the future
    pub poster_path: Option<String>,
}
