use anyhow::*;
use sqlx::sqlite::SqliteRow;
use sqlx::{Acquire, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::infrastructure::media_db::query_manager::QueryManager;
use crate::interfaces::dtos::MediaItemDto;

#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_library_medias(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaItemDto>,
    library_id: i64,
) -> Result<Vec<MediaItemDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager.get_query("media", "find_all_media").await?;
    let raw_media = sqlx::query(&query)
        .bind(library_id)
        .fetch_all(&mut *tx)
        .await?;

    Ok(mapper(raw_media))
}

#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_library_media(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaItemDto>,
    library_id: i64,
) -> Result<Vec<MediaItemDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager
        .get_query("media_item", "find_media_item_by_id")
        .await?;

    let series = sqlx::query(&query)
        .bind(library_id)
        .fetch_all(&mut *tx)
        .await?;

    Ok(mapper(series))
}

#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_library_media_episodes(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaItemDto>,
    library_id: i64,
    media_id: i64,
) -> Result<Vec<MediaItemDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager.get_query("media", "find_media_by_id").await?;
    let raw_media = sqlx::query(&query)
        .bind(library_id)
        .bind(media_id)
        .fetch_all(&mut *tx)
        .await?;

    Ok(mapper(raw_media))
}

#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_media_by_id(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaItemDto>,
    media_id: i64,
) -> Result<Option<MediaItemDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager
        .get_query("media_item", "find_media_item_by_id")
        .await?;
    let series = sqlx::query(&query)
        .bind(media_id)
        .fetch_all(&mut *tx)
        .await?;

    let results = mapper(series);
    Ok(results.into_iter().next())
}
