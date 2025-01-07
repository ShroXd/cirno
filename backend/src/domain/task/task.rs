use ambassador::delegatable_trait;
use anyhow::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task::JoinHandle};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    infrastructure::event_bus::event_bus::EventBus, interfaces::ws::notification::ToJsonPayload,
};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TaskType {
    MediaLibraryScan,
    PipelinePreparation,
    /// Test task used only in unit tests
    TestTask,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Queued,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, TS)]
#[ts(export)]
#[ts(type = "string")]
pub struct TaskId(pub Uuid);

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct AsyncTaskResponse<T = ()> {
    pub task_id: TaskId,
    pub task_type: TaskType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<T>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AsyncTaskProgress {
    pub task_id: TaskId,
    pub status: TaskStatus,
    pub progress: f32,
    pub message: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AsyncTaskEvent {
    Started {
        identifier: TaskIdentifier,
    },
    ProgressUpdated {
        identifier: TaskIdentifier,
        progress: f32,
    },
    Completed {
        identifier: TaskIdentifier,
    },
    Error {
        identifier: TaskIdentifier,
        error: String,
    },
}

impl ToJsonPayload for AsyncTaskEvent {
    fn to_json_payload(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct AsyncTaskInfo {
    pub id: TaskId,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub progress: f32,
    /// used to notify the client
    pub websocket_client_key: String,
    pub cleanup_handle: Option<Arc<Mutex<Option<JoinHandle<()>>>>>,
    pub retention_period: Duration,
}

#[async_trait]
pub trait AsyncTask: Send + Sync + TaskIdentifiable {
    async fn execute(&self, identifier: TaskIdentifier, event_bus: Arc<EventBus>) -> Result<()>;
}

#[delegatable_trait]
pub trait TaskIdentifiable {
    fn set_task_id(&mut self, task_id: TaskId);
    fn get_task_id(&self) -> TaskId;
    fn set_ws_client_id(&mut self, ws_client_id: String);
    fn get_ws_client_id(&self) -> Option<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TaskIdentifier {
    task_id: TaskId,
    ws_client_id: Option<String>,
}

impl TaskIdentifier {
    pub fn new(task_id: TaskId, ws_client_id: Option<String>) -> Self {
        Self {
            task_id,
            ws_client_id,
        }
    }
}

impl Display for TaskIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ws_client_display = match &self.ws_client_id {
            Some(id) => id.to_string(),
            None => "<no client>".to_string(),
        };

        write!(
            f,
            "Task[id: {}, client: {}]",
            self.task_id.0, ws_client_display
        )
    }
}

impl Default for TaskIdentifier {
    fn default() -> Self {
        Self {
            task_id: TaskId(Uuid::nil()),
            ws_client_id: None,
        }
    }
}

impl TaskIdentifiable for TaskIdentifier {
    fn set_task_id(&mut self, task_id: TaskId) {
        self.task_id = task_id;
    }

    fn get_task_id(&self) -> TaskId {
        self.task_id.clone()
    }

    fn set_ws_client_id(&mut self, ws_client_id: String) {
        self.ws_client_id = Some(ws_client_id);
    }

    fn get_ws_client_id(&self) -> Option<String> {
        self.ws_client_id.clone()
    }
}
