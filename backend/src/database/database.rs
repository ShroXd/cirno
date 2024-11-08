use anyhow::*;
use sqlx::SqlitePool;
use std::result::Result::Ok;
use tracing::*;

pub struct Database {
    file_path: &'static str,
    pool: SqlitePool,
}

impl Database {
    #[instrument]
    pub async fn new(file_path: &'static str) -> Result<Self> {
        // TODO: https://tauritutorials.com/blog/building-a-todo-app-in-tauri-with-sqlite-and-sqlx
        let pool = SqlitePool::connect(&format!("sqlite:{}", file_path)).await?;
        debug!("Initialized database at {}", file_path);

        Ok(Self { file_path, pool })
    }

    #[instrument(skip(self))]
    pub fn get_connection_pool(&self) -> SqlitePool {
        self.pool.clone()
    }
}
