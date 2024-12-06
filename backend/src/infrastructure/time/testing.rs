#[cfg(test)]
pub mod test {
    use crate::domain::time::TimeProvider;
    use async_trait::async_trait;
    use std::{sync::Arc, time::Duration};
    use tokio::sync::Notify;

    #[derive(Clone)]
    pub struct TestingTimeProvider {
        notify: Arc<Notify>,
    }

    impl TestingTimeProvider {
        pub fn new() -> Self {
            Self {
                notify: Arc::new(Notify::new()),
            }
        }

        pub fn get_notify(&self) -> Arc<Notify> {
            self.notify.clone()
        }
    }

    #[async_trait]
    impl TimeProvider for TestingTimeProvider {
        async fn sleep(&self, _duration: Duration) {
            self.notify.notify_waiters();
        }
    }
}
