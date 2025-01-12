use anyhow::*;
use std::{
    future::Future,
    pin::Pin,
    sync::{atomic::AtomicU32, Arc},
    time::Duration,
};
use tracing::*;

use super::{
    domain_event::DomainEvent,
    event_bus::EventBus,
    retry::{ExponentialRetryStrategy, FixedRetryStrategy, RetryStrategy},
};

#[derive(Clone, Default)]
pub struct EventHandlerConfig {
    pub timeout: Option<Duration>,
    pub retry_strategy: Option<Arc<dyn RetryStrategy>>,
    pub max_triggers: Option<u32>,
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

    /// Creates an event handler configuration with exponential backoff retry strategy
    ///
    /// ## Arguments
    ///
    /// * `initial_delay` - Initial delay between retries
    /// * `max_delay` - Maximum delay between retries
    /// * `max_attempts` - Maximum number of retry attempts
    /// * `multiplier` - Factor to multiply delay by after each attempt
    /// * `jitter` - Random factor to add to delay (0.0-1.0)
    ///
    /// ## Returns
    ///
    /// Returns an EventHandlerConfig with exponential retry strategy configured
    ///
    /// ## Example
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let config = EventHandlerConfig::with_exponential_retry(
    ///     Duration::from_secs(1),    // Start with 1 second delay
    ///     Duration::from_secs(30),   // Max delay of 30 seconds
    ///     3,                         // Retry up to 3 times
    ///     2.0,                       // Double the delay each time
    ///     0.1                        // Add up to 10% random jitter
    /// );
    /// ```
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
    dyn Fn(DomainEvent, Arc<EventBus>) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>
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
