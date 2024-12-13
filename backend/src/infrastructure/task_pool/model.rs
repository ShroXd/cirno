use ambassador::delegatable_trait;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task::JoinHandle};

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
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub id: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub progress: f32,
    /// used to notify the client
    pub websocket_client_key: String,
    pub cleanup_handle: Option<Arc<Mutex<Option<JoinHandle<()>>>>>,
    pub retention_period: Duration,
}

#[delegatable_trait]
pub trait TaskIdentifiable {
    fn set_task_id(&mut self, task_id: String);
    fn get_task_id(&self) -> String;
    fn set_ws_client_id(&mut self, ws_client_id: String);
    fn get_ws_client_id(&self) -> String;
}

#[async_trait]
pub trait AsyncTask: Send + Sync {
    async fn execute(
        &self,
        ws_client_id: String,
        task_id: String,
        event_bus: Arc<EventBus>,
    ) -> Result<()>;
    fn set_task_id(&mut self, task_id: String);
    fn get_task_id(&self) -> String;
    fn set_ws_client_id(&mut self, ws_client_id: String);
    fn get_ws_client_id(&self) -> String;
}

pub struct BaseTask {
    task_id: String,
    ws_client_id: String,
}

impl TaskIdentifiable for BaseTask {
    fn set_task_id(&mut self, task_id: String) {
        self.task_id = task_id;
    }

    fn get_task_id(&self) -> String {
        self.task_id.clone()
    }

    fn set_ws_client_id(&mut self, ws_client_id: String) {
        self.ws_client_id = ws_client_id;
    }

    fn get_ws_client_id(&self) -> String {
        self.ws_client_id.clone()
    }
}
