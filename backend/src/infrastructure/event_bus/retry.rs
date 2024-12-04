use rand::random;
use std::time::Duration;
use tracing::*;

pub trait RetryStrategy: Send + Sync + 'static {
    fn next_delay(&self, attempt: u32) -> Option<Duration>;
}

#[derive(Clone)]
pub struct FixedRetryStrategy {
    pub delay: Duration,
    pub max_attempts: u32,
}

impl RetryStrategy for FixedRetryStrategy {
    #[instrument(skip(self))]
    fn next_delay(&self, attempt: u32) -> Option<Duration> {
        if attempt >= self.max_attempts {
            None
        } else {
            Some(self.delay)
        }
    }
}

#[derive(Clone)]
pub struct ExponentialRetryStrategy {
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub max_attempts: u32,
    pub multiplier: f64,
    pub jitter: f64,
}

impl RetryStrategy for ExponentialRetryStrategy {
    #[instrument(skip(self))]
    fn next_delay(&self, attempt: u32) -> Option<Duration> {
        if attempt >= self.max_attempts {
            return None;
        }

        let base_delay =
            self.initial_delay.as_millis() as f64 * self.multiplier.powi(attempt as i32);
        debug!("Calculated base delay: {}ms", base_delay.round() as u64);

        let jitter_range = base_delay * self.jitter;
        let jitter_value = (random::<f64>() * 2.0 - 1.0) * jitter_range;
        debug!("Calculated jitter: {}ms", jitter_value.round() as u64);

        let delay = (base_delay + jitter_value)
            .min(self.max_delay.as_millis() as f64)
            .max(0.0);
        debug!("Calculated delay: {}ms", delay.round() as u64);

        Some(Duration::from_millis(delay as u64))
    }
}
