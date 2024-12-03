use anyhow::*;
use futures::future;
use std::result::Result::Ok;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};
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
        let tasks = Arc::new(RwLock::new(HashMap::<String, TaskInfo>::new()));
        let event_bus_clone = event_bus.clone();
        let tasks_clone = tasks.clone();

        debug!(
            "Task pool created with {} max concurrent tasks",
            max_concurrent_tasks
        );

        tokio::spawn(async move {
            let mut handles: Vec<JoinHandle<()>> = Vec::new();

            while let Some(task) = task_rx.recv().await {
                let task_id = task.get_task_id();
                let ws_client_id = task.get_ws_client_id();
                debug!("Task received: {}", task_id);
                debug!("Task ws client id: {}", ws_client_id);

                if handles.len() >= max_concurrent_tasks {
                    debug!("Max concurrent tasks reached, waiting for one to finish");
                    let _ = future::select_all(&mut handles).await;
                    handles.retain(|h| !h.is_finished());
                    debug!("One task finished, continuing");
                }

                let event_bus = event_bus_clone.clone();
                let tasks = tasks_clone.clone();

                handles.push(tokio::spawn(async move {
                    debug!("Executing task: {}", task_id);
                    let result = task
                        .execute(ws_client_id.clone(), task_id.clone(), event_bus)
                        .await;

                    let mut tasks_write = tasks.write().await;
                    if let Some(task_info) = tasks_write.get_mut(&task_id) {
                        task_info.status = match result {
                            Ok(_) => TaskStatus::Completed,
                            Err(e) => {
                                debug!("Task {} failed: {}", task_id, e);
                                TaskStatus::Failed(e.to_string())
                            }
                        };
                        task_info.progress = 100.0;
                        debug!("Task completed: {}", task_id);

                        debug!("Scheduling cleanup for task: {}", task_id);
                        let cleanup_handle = TaskPool::schedule_cleanup(
                            tasks.clone(),
                            task_id.clone(),
                            task_info.retention_period,
                        );
                        task_info.cleanup_handle = Some(Arc::new(Mutex::new(Some(cleanup_handle))));
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

    #[instrument(skip(tasks))]
    fn schedule_cleanup(
        tasks: Arc<RwLock<HashMap<String, TaskInfo>>>,
        task_id: String,
        retention_period: Duration,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            debug!("Waiting for cleanup for task: {}", task_id);
            tokio::time::sleep(retention_period).await;
            debug!("Cleanup for task: {}", task_id);
            tasks.write().await.remove(&task_id);
        })
    }

    #[instrument(skip(self, task))]
    pub async fn register_task(
        &self,
        task_type: TaskType,
        websocket_client_key: String,
        mut task: Box<dyn AsyncTask>,
        retention_period: Option<Duration>,
    ) -> Result<String> {
        let task_id = Uuid::new_v4().to_string();
        let task_id_clone = task_id.clone();
        task.set_task_id(task_id_clone.clone());

        debug!("Registering task: {}", task_id);

        let task_info = TaskInfo {
            id: task_id,
            task_type,
            status: TaskStatus::Pending,
            progress: 0.0,
            websocket_client_key,
            cleanup_handle: None,
            retention_period: retention_period.unwrap_or(Duration::from_secs(60 * 10)), // 10 minutes
        };

        debug!("Inserting task info into tasks map");
        self.tasks
            .write()
            .await
            .insert(task_id_clone.clone(), task_info);

        debug!("Sending task to task tx");
        self.task_tx.send(task).await?;

        Ok(task_id_clone)
    }

    #[instrument(skip(self))]
    pub async fn extend_retention(&self, task_id: String, extension: Duration) -> Result<()> {
        debug!("Extending retention for task: {}", task_id);
        let mut tasks = self.tasks.write().await;
        if let Some(task_info) = tasks.get_mut(&task_id) {
            if let Some(cleanup_handle) = &task_info.cleanup_handle {
                if let Some(handle) = cleanup_handle.lock().await.take() {
                    debug!("Aborting cleanup for task: {}", task_id);
                    handle.abort();
                }
            }

            let new_retention_period = task_info.retention_period + extension;
            debug!(
                "Scheduling new cleanup for task: {} with period: {:?}",
                task_id, new_retention_period
            );

            let new_cleanup_handle = TaskPool::schedule_cleanup(
                self.tasks.clone(),
                task_id.clone(),
                new_retention_period,
            );
            task_info.cleanup_handle = Some(Arc::new(Mutex::new(Some(new_cleanup_handle))));
            task_info.retention_period = new_retention_period;
            Ok(())
        } else {
            Err(anyhow!("Task not found"))
        }
    }

    #[instrument(skip(self))]
    pub async fn cleanup_task(&self, task_id: String) -> Result<()> {
        debug!("Cleaning up task: {}", task_id);
        let mut tasks = self.tasks.write().await;
        if let Some(task_info) = tasks.get_mut(&task_id) {
            if let Some(cleanup_handle) = &task_info.cleanup_handle {
                if let Some(handle) = cleanup_handle.lock().await.take() {
                    debug!("Aborting cleanup for task: {}", task_id);
                    handle.abort();
                }
            }

            debug!("Removing task from tasks map: {}", task_id);
            tasks.remove(&task_id);
            Ok(())
        } else {
            Err(anyhow!("Task not found"))
        }
    }

    // TODO: maybe we can try to reuse same resources for new task?

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
