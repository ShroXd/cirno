use anyhow::*;
use std::{
    future::Future,
    pin::Pin,
    sync::{atomic::AtomicU32, Arc},
    time::Duration,
};
use tracing::*;

use super::{
    event_bus::{DomainEvent, EventBus},
    retry::{ExponentialRetryStrategy, FixedRetryStrategy, RetryStrategy},
};

#[derive(Clone)]
pub struct EventHandlerConfig {
    pub timeout: Option<Duration>,
    pub retry_strategy: Option<Arc<dyn RetryStrategy>>,
    pub max_triggers: Option<u32>,
}

impl Default for EventHandlerConfig {
    fn default() -> Self {
        Self {
            timeout: None,
            retry_strategy: None,
            max_triggers: None,
        }
    }
}

impl EventHandlerConfig {
    #[instrument]
    pub fn with_fixed_retry(delay: Duration, max_attempts: u32) -> Self {
        Self {
            retry_strategy: Some(Arc::new(FixedRetryStrategy {
                delay,
                max_attempts,
            })),
            ..Default::default()
        }
    }

    #[instrument]
    pub fn with_exponential_retry(
        initial_delay: Duration,
        max_delay: Duration,
        max_attempts: u32,
        multiplier: f64,
        jitter: f64,
    ) -> Self {
        Self {
            retry_strategy: Some(Arc::new(ExponentialRetryStrategy {
                initial_delay,
                max_delay,
                max_attempts,
                multiplier,
                jitter,
            })),
            ..Default::default()
        }
    }

    pub fn one_time() -> Self {
        Self {
            max_triggers: Some(1),
            ..Default::default()
        }
    }

    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            timeout: Some(timeout),
            ..Default::default()
        }
    }

    pub fn with_max_triggers(max_triggers: u32) -> Self {
        Self {
            max_triggers: Some(max_triggers),
            ..Default::default()
        }
    }
}

pub type EventMatcher = Arc<dyn Fn(&DomainEvent) -> bool + Send + Sync + 'static>;
pub type HandlerFn = Arc<
    dyn Fn(DomainEvent, String, Arc<EventBus>) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub struct EventHandler {
    pub matcher: EventMatcher,
    pub handler: HandlerFn,
    pub config: EventHandlerConfig,
    pub trigger_count: Arc<AtomicU32>,
}
