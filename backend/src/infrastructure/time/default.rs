use async_trait::async_trait;
use std::time::Duration;

use crate::domain::time::TimeProvider;

#[derive(Clone)]
pub struct DefaultTimeProvider;

#[async_trait]
impl TimeProvider for DefaultTimeProvider {
    async fn sleep(&self, duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}
