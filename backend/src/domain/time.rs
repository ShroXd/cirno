use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait TimeProvider: Send + Sync + 'static {
    async fn sleep(&self, duration: Duration);
}
