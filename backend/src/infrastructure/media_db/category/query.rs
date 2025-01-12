use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::infrastructure::media_db::query_manager::QueryManager;

#[instrument(skip(conn_pool, query_manager))]
pub async fn check_category_exists(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    category_id: i64,
) -> Result<bool> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager
        .get_query("category", "check_category_exists")
        .await?;

    match sqlx::query_scalar::<_, i64>(&query)
        .bind(category_id)
        .fetch_optional(&mut *tx)
        .await?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
