use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum LibraryStatus {
    Pending,
    Scanning,
    Active,
    Inactive,
    Error,
}

// Mapping between the status and the id, same as the library_status table in the database
impl LibraryStatus {
    pub fn from_id(id: i64) -> Self {
        match id {
            1 => Self::Pending,
            2 => Self::Scanning,
            3 => Self::Active,
            4 => Self::Inactive,
            5 => Self::Error,
            _ => Self::Pending,
        }
    }

    pub fn to_id(&self) -> i64 {
        match self {
            Self::Pending => 1,
            Self::Scanning => 2,
            Self::Active => 3,
            Self::Inactive => 4,
            Self::Error => 5,
        }
    }
}
