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
    domain_event::DomainEvent,
    handler::{EventHandler, EventHandlerConfig},
};

#[derive(Clone)]
pub struct EventBus {
    tx: broadcast::Sender<DomainEvent>,
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

    pub fn subscribe(&self) -> broadcast::Receiver<DomainEvent> {
        self.tx.subscribe()
    }

    pub fn publish(&self, event: DomainEvent) -> Result<()> {
        self.tx
            .send(event)
            .map_err(|_| anyhow!("Failed to send event"))?;

        Ok(())
    }

    pub async fn on<F, Fut>(
        &self,
        matcher: impl Fn(&DomainEvent) -> bool + Send + Sync + 'static,
        handler: F,
        config: EventHandlerConfig,
    ) where
        F: Fn(DomainEvent, Arc<EventBus>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let handler = EventHandler {
            matcher: Arc::new(matcher),
            handler: Arc::new(move |event, event_bus| Box::pin(handler(event, event_bus))),
            config,
            trigger_count: Arc::new(AtomicU32::new(0)),
        };

        self.handlers.lock().await.push(handler);
    }

    pub fn start(&self) {
        let event_bus = Arc::new(self.clone());
        let mut rx = self.subscribe();

        spawn(async move {
            while let Ok(event) = rx.recv().await {
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
                    let event_bus_clone = event_bus.clone();

                    spawn(async move {
                        trigger_count.fetch_add(1, Ordering::SeqCst);
                        let mut attempt = 0;

                        loop {
                            let future = handler_fn(event.clone(), event_bus_clone.clone());

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

                            // TODO: investigate why send notification fails
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::dispatcher::model::GeneralEvent;
    use std::{sync::atomic::AtomicBool, time::Duration};
    use tokio::sync::Notify;

    fn create_test_event(id: i32) -> DomainEvent {
        DomainEvent::General(GeneralEvent::TestEvent(id))
    }

    #[tokio::test]
    async fn test_basic_publish_and_subscribe() {
        let event_bus = EventBus::new(16);
        let mut rx = event_bus.subscribe();

        let event = create_test_event(1);
        event_bus.publish(event.clone()).unwrap();

        if let Ok(received_event) = rx.recv().await {
            assert!(matches!(received_event, DomainEvent::General(_)));
        }
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let event_bus = EventBus::new(16);
        let mut rx1 = event_bus.subscribe();
        let mut rx2 = event_bus.subscribe();

        let event = create_test_event(1);
        event_bus.publish(event.clone()).unwrap();

        let event1 = rx1.recv().await.unwrap();
        let event2 = rx2.recv().await.unwrap();

        assert!(matches!(event1, DomainEvent::General(_)));
        assert!(matches!(event2, DomainEvent::General(_)));
    }

    #[tokio::test]
    async fn test_event_handler() {
        let event_bus = EventBus::new(16);
        let handled = Arc::new(AtomicBool::new(false));
        let completed = Arc::new(Notify::new());

        let handled_clone = handled.clone();
        let completed_clone = completed.clone();

        event_bus
            .on(
                |event| matches!(event, DomainEvent::General(_)),
                move |_, _| {
                    let handled = handled_clone.clone();
                    let completed = completed_clone.clone();

                    async move {
                        handled.store(true, Ordering::SeqCst);
                        completed.notify_one();

                        Ok(())
                    }
                },
                EventHandlerConfig::default(),
            )
            .await;

        event_bus.start();

        let event = create_test_event(1);
        event_bus.publish(event.clone()).unwrap();

        completed.notified().await;
        assert!(handled.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_handler_with_retry() {
        let event_bus = EventBus::new(16);
        let attempt_count = Arc::new(AtomicU32::new(0));
        let completed = Arc::new(Notify::new());

        let attempt_count_clone = attempt_count.clone();
        let completed_clone = completed.clone();

        event_bus
            .on(
                |event| matches!(event, DomainEvent::General(_)),
                move |_, _| {
                    let attempts = attempt_count_clone.clone();
                    let completed = completed_clone.clone();

                    async move {
                        let current = attempts.fetch_add(1, Ordering::SeqCst);
                        if current < 2 {
                            Err(anyhow!("Simulated failure"))
                        } else {
                            // Success after 2 attempts
                            completed.notify_one();
                            Ok(())
                        }
                    }
                },
                EventHandlerConfig::with_fixed_retry(Duration::from_millis(100), 3),
            )
            .await;

        event_bus.start();

        let event = create_test_event(1);
        event_bus.publish(event.clone()).unwrap();

        completed.notified().await;
        assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_handler_timeout() {
        let event_bus = EventBus::new(16);
        let completed = Arc::new(Notify::new());
        let completed_clone = completed.clone();

        event_bus
            .on(
                |event| matches!(event, DomainEvent::General(_)),
                move |_, _| {
                    let completed = completed_clone.clone();

                    async move {
                        let timeout_fut = tokio::time::sleep(Duration::from_secs(10));
                        tokio::pin!(timeout_fut);

                        completed.notify_one();
                        timeout_fut.await;
                        Ok(())
                    }
                },
                EventHandlerConfig::with_timeout(Duration::from_millis(100)),
            )
            .await;

        event_bus.start();

        let event = create_test_event(1);
        event_bus.publish(event.clone()).unwrap();

        completed.notified().await;

        let timeout_result =
            tokio::time::timeout(Duration::from_millis(150), completed.notified()).await;
        assert!(timeout_result.is_err(), "Handler should have timed out");
    }

    #[tokio::test]
    async fn test_max_triggers() {
        let event_bus = EventBus::new(16);
        let trigger_count = Arc::new(AtomicU32::new(0));
        let all_done = Arc::new(Notify::new());

        let trigger_count_clone = trigger_count.clone();
        let all_done_clone = all_done.clone();

        event_bus
            .on(
                |event| matches!(event, DomainEvent::General(_)),
                move |_, _| {
                    let trigger_count = trigger_count_clone.clone();
                    let all_done = all_done_clone.clone();

                    async move {
                        let current = trigger_count.fetch_add(1, Ordering::SeqCst);
                        if current == 2 {
                            // 3rd trigger
                            all_done.notify_one();
                        }

                        Ok(())
                    }
                },
                EventHandlerConfig::with_max_triggers(3),
            )
            .await;

        event_bus.start();

        let event = create_test_event(1);
        for _ in 0..3 {
            event_bus.publish(event.clone()).unwrap();
        }

        all_done.notified().await;
        assert_eq!(trigger_count.load(Ordering::SeqCst), 3);
    }
}
