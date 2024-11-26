use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::domain::tv_show::model::TvShow;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct MediaLibrary {
    // TODO: maybe generic type
    pub tv_show: Vec<TvShow>,
}
