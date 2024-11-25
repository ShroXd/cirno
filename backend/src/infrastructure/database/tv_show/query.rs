use anyhow::*;
use sqlx::sqlite::SqliteRow;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::interfaces::dtos::MediaItemDto;

// The media item domain represents the parent concept for all media types like movies, TV shows, videos etc.
// For the MVP, we only support TV shows, so the media item related code is placed here.
// In the future, this could be moved to a separate media_item module when adding support for other types.
// The current implementation maps TV shows to media items since they are currently the only supported type.

#[instrument(skip(conn_pool, mapper))]
pub async fn query_all_media_items(
    conn_pool: &SqlitePool,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaItemDto>,
) -> Result<Vec<MediaItemDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let media_items = sqlx::query(
        "
        SELECT ts.id, ts.title, ts.poster_path, ts.fanart_path, ts.country, ts.year, ts.plot,
               group_concat(g.name, ', ') AS genres
        FROM tv_series ts
        JOIN tv_series_genres tsg ON ts.id = tsg.series_id
        JOIN genres g ON tsg.genre_id = g.id
        GROUP BY ts.id, ts.title
        ",
    )
    .fetch_all(&mut *tx)
    .await?;

    Ok(mapper(media_items))
}

#[instrument(skip(conn_pool, mapper))]
pub async fn query_series_by_media_library_id(
    conn_pool: &SqlitePool,
    media_library_id: i64,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaItemDto>,
) -> Result<Vec<MediaItemDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let series = sqlx::query(
        "SELECT ts.id, ts.title, ts.poster_path, ts.fanart_path, ts.country, ts.year, ts.plot,
               group_concat(g.name, ', ') AS genres
        FROM tv_series ts
        JOIN tv_series_genres tsg ON ts.id = tsg.series_id
        JOIN genres g ON tsg.genre_id = g.id
        WHERE ts.media_library_id = ?
        GROUP BY ts.id, ts.title",
    )
    .bind(media_library_id)
    .fetch_all(&mut *tx)
    .await?;

    Ok(mapper(series))
}
