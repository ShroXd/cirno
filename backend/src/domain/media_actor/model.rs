use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Actor {
    pub name: Option<String>,
    pub role: Option<String>,
    pub thumb: Option<String>,
    pub profile: Option<String>,
    pub tmdb_id: Option<String>,
}
impl Default for Actor {
    fn default() -> Self {
        Actor {
            name: None,
            role: None,
            thumb: None,
            profile: None,
            tmdb_id: None,
        }
    }
}
