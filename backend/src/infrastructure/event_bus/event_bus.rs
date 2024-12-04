use anyhow::*;
use std::{
    future::Future,
    result::Result::Ok,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};
use tokio::{
    spawn,
    sync::{broadcast, Mutex},
    time::{sleep, timeout},
};
use tracing::*;

use super::{
    handler::{EventHandler, EventHandlerConfig},
    model::GeneralEvent,
};
use crate::domain::{
    media_library::event::MediaLibraryEventType, pipeline::event::PipelineEvent,
    websocket::event::WebSocketEvent,
};

#[derive(Debug, Clone)]
pub enum DomainEvent {
    General(GeneralEvent),
    MediaLibrary(MediaLibraryEventType),
    Pipeline(PipelineEvent),
    WebSocket(WebSocketEvent),
}

#[derive(Clone)]
pub struct EventBus {
    tx: broadcast::Sender<(DomainEvent, String)>, // (event, task id)
    handlers: Arc<Mutex<Vec<EventHandler>>>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            tx,
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<(DomainEvent, String)> {
        self.tx.subscribe()
    }

    pub fn publish(&self, event: DomainEvent, task_id: String) -> Result<()> {
        self.tx
            .send((event, task_id))
            .map_err(|_| anyhow!("Failed to send event"))?;

        Ok(())
    }

    pub async fn on<F, Fut>(
        &self,
        matcher: impl Fn(&DomainEvent) -> bool + Send + Sync + 'static,
        handler: F,
        config: EventHandlerConfig,
    ) where
        F: Fn(DomainEvent, String, Arc<EventBus>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let handler = EventHandler {
            matcher: Arc::new(matcher),
            handler: Arc::new(move |event, task_id, event_bus| {
                Box::pin(handler(event, task_id, event_bus))
            }),
            config,
            trigger_count: Arc::new(AtomicU32::new(0)),
        };

        self.handlers.lock().await.push(handler);
    }

    pub fn start(&self) {
        let event_bus = Arc::new(self.clone());
        let mut rx = self.subscribe();

        spawn(async move {
            while let Ok((event, task_id)) = rx.recv().await {
                let handlers = event_bus.handlers.lock().await;
                let mut to_remove = Vec::new();

                for (index, handler) in handlers.iter().enumerate() {
                    if !(handler.matcher)(&event) {
                        continue;
                    }

                    if let Some(max_triggers) = handler.config.max_triggers {
                        if handler.trigger_count.load(Ordering::Relaxed) >= max_triggers {
                            to_remove.push(index);
                            continue;
                        }
                    }

                    let handler_fn = handler.handler.clone();
                    let config = handler.config.clone();
                    let trigger_count = handler.trigger_count.clone();
                    let event = event.clone();
                    let task_id = task_id.clone();
                    let event_bus_clone = event_bus.clone();

                    spawn(async move {
                        trigger_count.fetch_add(1, Ordering::SeqCst);
                        let mut attempt = 0;

                        loop {
                            let future =
                                handler_fn(event.clone(), task_id.clone(), event_bus_clone.clone());

                            let result = match config.timeout {
                                Some(duration) => match timeout(duration, future).await {
                                    Ok(result) => result,
                                    Err(_) => {
                                        error!("Handler timed out");
                                        break;
                                    }
                                },
                                None => future.await,
                            };

                            if result.is_ok() {
                                break;
                            }

                            if let Some(ref strategy) = config.retry_strategy {
                                if let Some(delay) = strategy.next_delay(attempt) {
                                    attempt += 1;
                                    sleep(delay).await;
                                    continue;
                                }
                            }

                            error!("Handler failed after {} attempts", attempt + 1);
                            break;
                        }
                    });
                }
            }
        });
    }
}
