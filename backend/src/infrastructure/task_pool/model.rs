use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::infrastructure::event_bus::event_bus::EventBus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    MediaLibraryScan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: String,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub progress: f32,
    /// used to notify the client
    pub websocket_client_key: String,
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
