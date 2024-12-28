use anyhow::*;
use sqlx::SqlitePool;
use std::{result::Result::Ok, sync::Arc};
use tracing::*;

use super::query_manager::QueryManager;

pub struct Database {
    pool: SqlitePool,
    query_manager: Arc<dyn QueryManager>,
}

impl Database {
    #[instrument(skip(query_manager))]
    pub async fn new(database_url: &str, query_manager: Arc<dyn QueryManager>) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;

        Ok(Self {
            pool,
            query_manager,
        })
    }

    #[instrument(skip(self))]
    pub fn get_connection_pool(&self) -> SqlitePool {
        self.pool.clone()
    }

    #[instrument(skip(self))]
    pub fn get_query_manager(&self) -> Arc<dyn QueryManager> {
        self.query_manager.clone()
    }
}
