use std::sync::Arc;

use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::infrastructure::database::query_manager::QueryManager;

#[instrument(skip(conn_pool, query_manager))]
pub async fn delete_library(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    id: i64,
) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager.get_query("library", "delete_library").await?;

    sqlx::query(&query).bind(id).execute(&mut *tx).await?;

    tx.commit().await?;

    Ok(())
}
