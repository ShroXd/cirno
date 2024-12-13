use anyhow::*;
use futures::future;
use std::result::Result::Ok;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio::{sync::RwLock, task::JoinHandle};
use tracing::*;
use uuid::Uuid;

use crate::domain::task::task::{AsyncTask, TaskId};
use crate::{
    domain::{
        task::task::{
            ambassador_impl_TaskIdentifiable, AsyncTaskInfo, TaskIdentifiable, TaskStatus, TaskType,
        },
        time::TimeProvider,
    },
    infrastructure::{event_bus::event_bus::EventBus, time::default::DefaultTimeProvider},
};

#[derive(Clone)]
pub struct TaskPool {
    tasks: Arc<RwLock<HashMap<TaskId, AsyncTaskInfo>>>,
    event_bus: Arc<EventBus>,
    task_tx: mpsc::Sender<Box<dyn AsyncTask>>,
}

impl TaskPool {
    #[instrument(skip(event_bus))]
    pub fn new(max_concurrent_tasks: usize, event_bus: Arc<EventBus>) -> Self {
        Self::new_with_time_provider(
            max_concurrent_tasks,
            event_bus,
            Arc::new(DefaultTimeProvider),
        )
    }

    #[instrument(skip(event_bus, time_provider))]
    pub fn new_with_time_provider(
        max_concurrent_tasks: usize,
        event_bus: Arc<EventBus>,
        time_provider: Arc<dyn TimeProvider>,
    ) -> Self {
        let (task_tx, mut task_rx) = mpsc::channel::<Box<dyn AsyncTask>>(100);
        let tasks = Arc::new(RwLock::new(HashMap::<TaskId, AsyncTaskInfo>::new()));
        let event_bus_clone = event_bus.clone();
        let tasks_clone = tasks.clone();
        let time_provider_clone = time_provider.clone();

        debug!(
            "Task pool created with {} max concurrent tasks",
            max_concurrent_tasks
        );

        tokio::spawn(async move {
            let mut handles: Vec<JoinHandle<()>> = Vec::new();

            while let Some(task) = task_rx.recv().await {
                let task_id = task.get_task_id();
                let ws_client_id = task.get_ws_client_id();
                debug!("Task received: {:?}", task_id);
                debug!("Task ws client id: {}", ws_client_id);

                if handles.len() >= max_concurrent_tasks {
                    debug!("Max concurrent tasks reached, waiting for one to finish");
                    let _ = future::select_all(&mut handles).await;
                    handles.retain(|h| !h.is_finished());
                    debug!("One task finished, continuing");
                }

                let event_bus = event_bus_clone.clone();
                let tasks = tasks_clone.clone();
                let time_provider = time_provider_clone.clone();

                handles.push(tokio::spawn(async move {
                    debug!("Executing task: {:?}", task_id);
                    let result = task
                        .execute(ws_client_id.clone(), task_id.clone(), event_bus)
                        .await;

                    let mut tasks_write = tasks.write().await;
                    if let Some(task_info) = tasks_write.get_mut(&task_id) {
                        task_info.status = match result {
                            Ok(_) => TaskStatus::Completed,
                            Err(e) => {
                                debug!("Task {:?} failed: {}", task_id, e);
                                TaskStatus::Failed
                            }
                        };
                        task_info.progress = 100.0;
                        debug!("Task completed: {:?}", task_id);

                        debug!("Scheduling cleanup for task: {:?}", task_id);
                        let cleanup_handle = TaskPool::schedule_cleanup(
                            tasks.clone(),
                            task_id.clone(),
                            task_info.retention_period,
                            time_provider.clone(),
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

    #[instrument(skip(tasks, time_provider))]
    fn schedule_cleanup(
        tasks: Arc<RwLock<HashMap<TaskId, AsyncTaskInfo>>>,
        task_id: TaskId,
        retention_period: Duration,
        time_provider: Arc<dyn TimeProvider>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            debug!("Waiting for cleanup for task: {:?}", task_id);
            time_provider.sleep(retention_period).await;
            debug!("Cleanup for task: {:?}", task_id);
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
    ) -> Result<TaskId> {
        let task_id = Uuid::new_v4();
        let task_id_clone = task_id.clone();
        task.set_task_id(TaskId(task_id_clone));

        debug!("Registering task: {}", task_id);

        let task_info = AsyncTaskInfo {
            id: TaskId(task_id),
            task_type,
            status: TaskStatus::Queued,
            progress: 0.0,
            websocket_client_key,
            cleanup_handle: None,
            retention_period: retention_period.unwrap_or(Duration::from_secs(60 * 10)), // 10 minutes
        };

        debug!("Inserting task info into tasks map");
        self.tasks.write().await.insert(TaskId(task_id), task_info);

        debug!("Sending task to task tx");
        self.task_tx.send(task).await?;

        Ok(TaskId(task_id))
    }

    #[instrument(skip(self))]
    pub async fn extend_retention(&self, task_id: TaskId, extension: Duration) -> Result<()> {
        self.extend_retention_with_time_provider(task_id, extension, Arc::new(DefaultTimeProvider))
            .await
    }

    #[instrument(skip(self, time_provider))]
    pub async fn extend_retention_with_time_provider(
        &self,
        task_id: TaskId,
        extension: Duration,
        time_provider: Arc<dyn TimeProvider>,
    ) -> Result<()> {
        debug!("Extending retention for task: {:?}", task_id);
        let mut tasks = self.tasks.write().await;
        if let Some(task_info) = tasks.get_mut(&task_id) {
            if let Some(cleanup_handle) = &task_info.cleanup_handle {
                if let Some(handle) = cleanup_handle.lock().await.take() {
                    debug!("Aborting cleanup for task: {:?}", task_id);
                    handle.abort();
                }
            }

            let new_retention_period = task_info.retention_period + extension;
            debug!(
                "Scheduling new cleanup for task: {:?} with period: {:?}",
                task_id, new_retention_period
            );

            let new_cleanup_handle = TaskPool::schedule_cleanup(
                self.tasks.clone(),
                task_id,
                new_retention_period,
                time_provider,
            );
            task_info.cleanup_handle = Some(Arc::new(Mutex::new(Some(new_cleanup_handle))));
            task_info.retention_period = new_retention_period;
            Ok(())
        } else {
            Err(anyhow!("Task not found"))
        }
    }

    #[instrument(skip(self))]
    pub async fn cleanup_task(&self, task_id: TaskId) -> Result<()> {
        debug!("Cleaning up task: {:?}", task_id);
        let mut tasks = self.tasks.write().await;
        if let Some(task_info) = tasks.get_mut(&task_id) {
            if let Some(cleanup_handle) = &task_info.cleanup_handle {
                if let Some(handle) = cleanup_handle.lock().await.take() {
                    debug!("Aborting cleanup for task: {:?}", task_id);
                    handle.abort();
                }
            }

            debug!("Removing task from tasks map: {:?}", task_id);
            tasks.remove(&task_id);
            Ok(())
        } else {
            Err(anyhow!("Task not found"))
        }
    }

    // TODO: maybe we can try to reuse same resources for new task?

    #[instrument(skip(self))]
    pub async fn get_task_status(&self, task_id: TaskId) -> Result<TaskStatus> {
        self.tasks
            .read()
            .await
            .get(&task_id)
            .map(|t| t.status.clone())
            .ok_or(anyhow!("Task not found"))
    }

    #[instrument(skip(self))]
    pub async fn get_task_progress(&self, task_id: TaskId) -> Result<f32> {
        self.tasks
            .read()
            .await
            .get(&task_id)
            .map(|t| t.progress)
            .ok_or(anyhow!("Task not found"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::task::task::BaseTask, infrastructure::time::testing::test::TestingTimeProvider,
    };
    use ambassador::Delegate;
    use async_trait::async_trait;
    use tokio::sync::Notify;

    use super::*;

    #[derive(Delegate)]
    #[delegate(TaskIdentifiable, target = "base")]
    struct MockTask {
        base: BaseTask,
        should_fail: bool,
        completed: Arc<Notify>,
    }

    impl MockTask {
        fn new(task_id: TaskId, ws_client_id: String, should_fail: bool) -> Self {
            let completed = Arc::new(Notify::new());

            Self {
                base: BaseTask::default(),
                should_fail,
                completed,
            }
        }

        fn get_completed(&self) -> Arc<Notify> {
            self.completed.clone()
        }
    }

    #[async_trait]
    impl AsyncTask for MockTask {
        async fn execute(
            &self,
            _ws_client_id: String,
            _task_id: TaskId,
            _event_bus: Arc<EventBus>,
        ) -> Result<()> {
            self.completed.notify_one();
            match self.should_fail {
                true => Err(anyhow!("Task failed")),
                false => Ok(()),
            }
        }
    }

    #[tokio::test]
    async fn test_register_task() {
        let event_bus = Arc::new(EventBus::new(16));
        let pool = TaskPool::new(5, event_bus);

        let task_id = TaskId(Uuid::new_v4());
        let task = Box::new(MockTask::new(task_id, "client1".to_string(), false));
        let task_id = pool
            .register_task(TaskType::TestTask, "client1".to_string(), task, None)
            .await
            .unwrap();

        assert!(pool.tasks.read().await.contains_key(&task_id));
    }

    #[tokio::test]
    async fn test_task_completion() {
        let event_bus = Arc::new(EventBus::new(16));
        let pool = TaskPool::new(5, event_bus);

        let task_id = TaskId(Uuid::new_v4());
        let task = Box::new(MockTask::new(task_id, "client1".to_string(), false));
        let completed = task.get_completed();
        let task_id = pool
            .register_task(TaskType::TestTask, "client1".to_string(), task, None)
            .await
            .unwrap();

        completed.notified().await;

        let status = pool.get_task_status(task_id).await.unwrap();
        assert!(matches!(status, TaskStatus::Completed));
    }

    #[tokio::test]
    async fn test_task_failure() {
        let event_bus = Arc::new(EventBus::new(16));
        let pool = TaskPool::new(5, event_bus);

        let task_id = TaskId(Uuid::new_v4());
        let task = Box::new(MockTask::new(task_id, "client1".to_string(), true));
        let completed = task.get_completed();
        let task_id = pool
            .register_task(TaskType::TestTask, "client1".to_string(), task, None)
            .await
            .unwrap();

        completed.notified().await;

        let status = pool.get_task_status(task_id).await.unwrap();
        assert!(matches!(status, TaskStatus::Failed));
    }

    #[tokio::test(start_paused = true)]
    async fn test_scheduled_cleanup() {
        let event_bus = Arc::new(EventBus::new(16));
        let time_provider = Arc::new(TestingTimeProvider::new());
        let pool = TaskPool::new_with_time_provider(5, event_bus, time_provider);

        let task_id = TaskId(Uuid::new_v4());
        let task = Box::new(MockTask::new(task_id, "client1".to_string(), false));
        let completed = task.get_completed();
        let _ = pool
            .register_task(
                TaskType::TestTask,
                "client1".to_string(),
                task,
                Some(Duration::from_secs(60)),
            )
            .await
            .unwrap();

        completed.notified().await;

        assert!(pool.tasks.read().await.is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn test_task_retention() {
        let event_bus = Arc::new(EventBus::new(16));
        let time_provider = Arc::new(TestingTimeProvider::new());
        let pool = TaskPool::new_with_time_provider(5, event_bus, time_provider);

        let task_id = TaskId(Uuid::new_v4());
        let task = Box::new(MockTask::new(task_id, "client1".to_string(), false));
        let completed = task.get_completed();
        let task_id = pool
            .register_task(
                TaskType::TestTask,
                "client1".to_string(),
                task,
                Some(Duration::from_secs(60)),
            )
            .await
            .unwrap();

        pool.extend_retention(task_id.clone(), Duration::from_secs(30))
            .await
            .unwrap();

        completed.notified().await;

        assert!(pool.tasks.read().await.is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn test_cleanup_task() {
        let event_bus = Arc::new(EventBus::new(16));
        let time_provider = Arc::new(TestingTimeProvider::new());
        let pool = TaskPool::new_with_time_provider(5, event_bus, time_provider);

        let task_id = TaskId(Uuid::new_v4());
        let task = Box::new(MockTask::new(task_id, "client1".to_string(), false));
        let task_id = pool
            .register_task(TaskType::TestTask, "client1".to_string(), task, None)
            .await
            .unwrap();

        assert!(pool.tasks.read().await.contains_key(&task_id));
        pool.cleanup_task(task_id).await.unwrap();

        assert!(pool.tasks.read().await.is_empty());
    }
}
