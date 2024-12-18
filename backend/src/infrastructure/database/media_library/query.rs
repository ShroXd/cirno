use anyhow::*;
use sqlx::{sqlite::SqliteRow, Acquire, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::{
    domain::media_library::model::{MediaLibraryBrief, MediaLibraryPoster},
    infrastructure::database::query_manager::QueryManager,
};

// TODO: after finishing the user system, query the media libraries for the current user
#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_media_library(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    id: Option<i64>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaLibraryBrief>,
) -> Result<Vec<MediaLibraryBrief>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let raw_media_library = match id {
        Some(id) => {
            let query = query_manager
                .get_query("media_library", "find_media_library_by_id")
                .await?;
            sqlx::query(&query).bind(id).fetch_all(&mut *tx).await?
        }
        None => {
            let query = query_manager
                .get_query("media_library", "find_all_media_libraries")
                .await?;
            sqlx::query(&query).fetch_all(&mut *tx).await?
        }
    };

    Ok(mapper(raw_media_library))
}

#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_media_library_posters(
    media_library_id: i64,
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaLibraryPoster>,
) -> Result<Vec<MediaLibraryPoster>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    debug!("Querying media library posters for {}", media_library_id);

    let query = query_manager
        .get_query("media_library", "get_media_library_posters")
        .await?;

    let raw_media_library_posters: Vec<SqliteRow> = sqlx::query(&query)
        .bind(media_library_id)
        .fetch_all(&mut *tx)
        .await?;

    Ok(mapper(raw_media_library_posters))
}
