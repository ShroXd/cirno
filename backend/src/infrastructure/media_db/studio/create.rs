use std::sync::Arc;

use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::infrastructure::media_db::query_manager::QueryManager;

#[instrument(skip(conn_pool, query_manager))]
pub async fn save_studio(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    tv_show_id: i64,
    studio: String,
) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let save_studio_query = query_manager.get_query("studio", "save_studio").await?;
    let studio_id: i64 = sqlx::query_scalar(&save_studio_query)
        .bind(studio)
        .fetch_one(&mut *tx)
        .await?;

    let save_tv_show_studio_query = query_manager
        .get_query("studio", "save_tv_show_studio")
        .await?;
    sqlx::query(&save_tv_show_studio_query)
        .bind(tv_show_id)
        .bind(studio_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}
