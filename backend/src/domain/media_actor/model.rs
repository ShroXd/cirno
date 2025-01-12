use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Default, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct MediaActor {
    pub name: Option<String>,
    pub role: Option<String>,
    pub thumb: Option<String>,
    pub profile: Option<String>,
    pub tmdb_id: Option<String>,
}
