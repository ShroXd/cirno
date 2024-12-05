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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_fixed_retry_strategy() {
        let strategy = FixedRetryStrategy {
            delay: Duration::from_secs(1),
            max_attempts: 3,
        };

        // Successive calls should return the same delay
        assert_eq!(strategy.next_delay(0), Some(Duration::from_secs(1)));
        assert_eq!(strategy.next_delay(1), Some(Duration::from_secs(1)));
        assert_eq!(strategy.next_delay(2), Some(Duration::from_secs(1)));

        // After max attempts, no delay should be returned
        assert_eq!(strategy.next_delay(3), None);
        assert_eq!(strategy.next_delay(4), None);
    }

    #[test]
    fn test_exponential_retry_strategy_basic() {
        let strategy = ExponentialRetryStrategy {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            max_attempts: 3,
            multiplier: 2.0,
            jitter: 0.0,
        };

        let first_delay = strategy.next_delay(0).unwrap();
        assert!(first_delay >= Duration::from_millis(100));

        let second_delay = strategy.next_delay(1).unwrap();
        assert!(second_delay >= Duration::from_millis(200));

        let third_delay = strategy.next_delay(2).unwrap();
        assert!(third_delay >= Duration::from_millis(400));

        // After max attempts, no delay should be returned
        assert_eq!(strategy.next_delay(3), None);
        assert_eq!(strategy.next_delay(4), None);
    }

    #[test]
    fn test_exponential_retry_strategy_max_delay() {
        let strategy = ExponentialRetryStrategy {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(300),
            max_attempts: 5,
            multiplier: 2.0,
            jitter: 0.0,
        };

        assert_eq!(strategy.next_delay(0), Some(Duration::from_millis(100)));
        assert_eq!(strategy.next_delay(1), Some(Duration::from_millis(200)));

        // Third attempt should be capped at max delay (would have been 400)
        assert_eq!(strategy.next_delay(2), Some(Duration::from_millis(300)));
        assert_eq!(strategy.next_delay(3), Some(Duration::from_millis(300)));
        assert_eq!(strategy.next_delay(4), Some(Duration::from_millis(300)));
        assert_eq!(strategy.next_delay(5), None);
    }

    #[test]
    fn test_exponential_retry_strategy_with_jitter() {
        let strategy = ExponentialRetryStrategy {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            max_attempts: 5,
            multiplier: 2.0,
            jitter: 0.5,
        };

        // Test first attempt (base delay: 100ms)
        let first_delay = strategy.next_delay(0).unwrap();
        assert!(
            first_delay >= Duration::from_millis(50), // 100ms * 0.5
            "First delay {:?} should be >= 50ms",
            first_delay
        );
        assert!(
            first_delay <= Duration::from_millis(150), // 100ms * 1.5
            "First delay {:?} should be <= 150ms",
            first_delay
        );

        // Test second attempt (base delay: 200ms)
        let second_delay = strategy.next_delay(1).unwrap();
        assert!(
            second_delay >= Duration::from_millis(100), // 200ms * 0.5
            "Second delay {:?} should be >= 100ms",
            second_delay
        );
        assert!(
            second_delay <= Duration::from_millis(300), // 200ms * 1.5
            "Second delay {:?} should be <= 300ms",
            second_delay
        );

        // Test third attempt (base delay: 400ms)
        let third_delay = strategy.next_delay(2).unwrap();
        assert!(
            third_delay >= Duration::from_millis(200), // 400ms * 0.5
            "Third delay {:?} should be >= 200ms",
            third_delay
        );
        assert!(
            third_delay <= Duration::from_millis(600), // 400ms * 1.5
            "Third delay {:?} should be <= 600ms",
            third_delay
        );
    }

    #[test]
    fn test_exponential_retry_strategy_with_zero_jitter() {
        let strategy = ExponentialRetryStrategy {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            max_attempts: 5,
            multiplier: 2.0,
            jitter: 0.0,
        };

        assert_eq!(strategy.next_delay(0), Some(Duration::from_millis(100)));
        assert_eq!(strategy.next_delay(1), Some(Duration::from_millis(200)));
        assert_eq!(strategy.next_delay(2), Some(Duration::from_millis(400)));
        assert_eq!(strategy.next_delay(3), Some(Duration::from_millis(800)));
        assert_eq!(strategy.next_delay(4), Some(Duration::from_millis(1600)));
        assert_eq!(strategy.next_delay(5), None);
    }
}
