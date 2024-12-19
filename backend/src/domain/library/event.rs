use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::model::Library;
use crate::{
    define_payload, domain::task::task::TaskIdentifier, interfaces::ws::notification::ToJsonPayload,
};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum LibraryEventType {
    LibraryScanned {
        task_identifier: TaskIdentifier,
        library: Library,
    },
    LibrarySaved {
        task_identifier: TaskIdentifier,
        library_id: i64,
        library_name: String,
    },
}

define_payload!(LibraryScanned {
    task_identifier: TaskIdentifier,
    library: Library,
});

define_payload!(LibrarySaved {
    task_identifier: TaskIdentifier,
    library_id: i64,
    library_name: String,
});

impl ToJsonPayload for LibraryEventType {
    fn to_json_payload(&self) -> serde_json::Value {
        match self {
            LibraryEventType::LibraryScanned {
                task_identifier,
                library,
            } => {
                serde_json::json!(LibraryScanned::new(
                    task_identifier.to_owned(),
                    library.to_owned()
                ))
            }
            LibraryEventType::LibrarySaved {
                task_identifier,
                library_id,
                library_name,
            } => {
                serde_json::json!(LibrarySaved::new(
                    task_identifier.to_owned(),
                    library_id.to_owned(),
                    library_name.to_owned()
                ))
            }
        }
    }
}
