use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

#[instrument(skip(conn_pool, id))]
pub async fn delete_library(conn_pool: &SqlitePool, id: i64) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    sqlx::query!("DELETE FROM library WHERE id = ?", id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}
