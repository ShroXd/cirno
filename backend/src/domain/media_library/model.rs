use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::domain::tv_show::model::TVSerie;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct MediaLibrary {
    // TODO: maybe generic type
    pub series: Vec<TVSerie>,
}
