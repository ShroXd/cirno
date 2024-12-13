use ambassador::delegatable_trait;
use anyhow::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task::JoinHandle};
use uuid::Uuid;

use crate::infrastructure::event_bus::event_bus::EventBus;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskId(pub Uuid);

#[derive(Debug, Clone, Serialize)]
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
    async fn execute(
        &self,
        ws_client_id: String,
        task_id: TaskId,
        event_bus: Arc<EventBus>,
    ) -> Result<()>;
}

#[delegatable_trait]
pub trait TaskIdentifiable {
    fn set_task_id(&mut self, task_id: TaskId);
    fn get_task_id(&self) -> TaskId;
    fn set_ws_client_id(&mut self, ws_client_id: String);
    fn get_ws_client_id(&self) -> String;
}

pub struct BaseTask {
    task_id: TaskId,
    ws_client_id: String,
}

impl Default for BaseTask {
    fn default() -> Self {
        Self {
            task_id: TaskId(Uuid::nil()),
            ws_client_id: "".to_string(),
        }
    }
}

impl TaskIdentifiable for BaseTask {
    fn set_task_id(&mut self, task_id: TaskId) {
        self.task_id = task_id;
    }

    fn get_task_id(&self) -> TaskId {
        self.task_id.clone()
    }

    fn set_ws_client_id(&mut self, ws_client_id: String) {
        self.ws_client_id = ws_client_id;
    }

    fn get_ws_client_id(&self) -> String {
        self.ws_client_id.clone()
    }
}
