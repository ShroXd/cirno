use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

#[instrument(skip(conn_pool))]
pub async fn check_category_exists(conn_pool: &SqlitePool, category_id: i64) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    match sqlx::query_scalar!("SELECT id FROM category_mapping WHERE id = ?", category_id,)
        .fetch_optional(&mut *tx)
        .await?
    {
        Some(_) => Ok(()),
        None => Err(anyhow::anyhow!("Category does not exist")),
    }
}
