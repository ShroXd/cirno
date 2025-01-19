use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct File {
    id: String, // uuid
    pub path: PathBuf,
}

impl File {
    pub fn new(path: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            path,
        }
    }
}
