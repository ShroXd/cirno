use std::sync::Arc;

use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::infrastructure::database::query_manager::QueryManager;

#[instrument(skip(conn_pool, query_manager))]
pub async fn save_genre(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    series_id: i64,
    genre: String,
) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let save_genre_query = query_manager.get_query("genre", "save_genre").await?;
    let genre_id: i64 = sqlx::query_scalar(&save_genre_query)
        .bind(genre)
        .fetch_one(&mut *tx)
        .await?;

    let save_tv_show_genre_query = query_manager
        .get_query("genre", "save_tv_show_genre")
        .await?;
    sqlx::query(&save_tv_show_genre_query)
        .bind(series_id)
        .bind(genre_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}
