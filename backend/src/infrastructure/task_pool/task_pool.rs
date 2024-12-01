use anyhow::*;
use futures::future;
use std::result::Result::Ok;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc;
use tokio::{
    sync::{broadcast, RwLock},
    task::JoinHandle,
};
use tracing::*;
use uuid::Uuid;

use super::model::{AsyncTask, TaskInfo, TaskStatus, TaskType};
use crate::infrastructure::event_bus::event_bus::{DomainEvent, EventBus};

#[derive(Debug, Clone)]
pub struct TaskPool {
    tasks: Arc<RwLock<HashMap<String, TaskInfo>>>,
    event_bus: Arc<EventBus>,
    task_tx: mpsc::Sender<Box<dyn AsyncTask>>,
}

impl TaskPool {
    #[instrument]
    pub fn new(max_concurrent_tasks: usize, event_bus: Arc<EventBus>) -> Self {
        let (task_tx, mut task_rx) = mpsc::channel::<Box<dyn AsyncTask>>(100);
        let tasks = Arc::new(RwLock::new(HashMap::new()));

        let event_bus_clone = event_bus.clone();

        tokio::spawn(async move {
            let mut handles: Vec<JoinHandle<()>> = Vec::new();

            while let Some(task) = task_rx.recv().await {
                let task_id = task.get_task_id();
                let ws_client_id = task.get_ws_client_id();

                if handles.len() >= max_concurrent_tasks {
                    let _ = future::select_all(&mut handles).await;
                    handles.retain(|h| !h.is_finished());
                }
                let event_bus = event_bus_clone.clone();
                handles.push(tokio::spawn(async move {
                    if let Err(e) = task.execute(ws_client_id, task_id, event_bus).await {
                        error!("Async task execution failed: {:?}", e);
                    }
                }));
            }
        });

        Self {
            tasks,
            event_bus,
            task_tx,
        }
    }

    #[instrument(skip(self, task))]
    pub async fn register_task(
        &self,
        task_type: TaskType,
        websocket_client_key: String,
        mut task: Box<dyn AsyncTask>,
    ) -> Result<String> {
        let task_id = Uuid::new_v4().to_string();
        let task_id_clone = task_id.clone();

        task.set_task_id(task_id_clone.clone());

        let task_info = TaskInfo {
            id: task_id,
            task_type,
            status: TaskStatus::Pending,
            progress: 0.0,
            websocket_client_key,
        };

        self.tasks
            .write()
            .await
            .insert(task_id_clone.clone(), task_info);

        let _ = self.task_tx.send(task).await;

        Ok(task_id_clone)
    }

    #[instrument(skip(self))]
    pub async fn get_task_status(&self, task_id: String) -> Result<TaskStatus> {
        self.tasks
            .read()
            .await
            .get(&task_id)
            .map(|t| t.status.clone())
            .ok_or(anyhow!("Task not found"))
    }

    #[instrument(skip(self))]
    pub async fn get_task_progress(&self, task_id: String) -> Result<f32> {
        self.tasks
            .read()
            .await
            .get(&task_id)
            .map(|t| t.progress)
            .ok_or(anyhow!("Task not found"))
    }
}
