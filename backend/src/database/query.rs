use anyhow::*;
use sqlx::sqlite::SqliteRow;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::interfaces::dtos::{EpisodeDto, MediaItemDto, MediaLibraryDto, SeasonDto};

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

#[instrument(skip(conn_pool, mapper))]
pub async fn query_seasons(
    conn_pool: &SqlitePool,
    series_id: i64,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<SeasonDto>,
) -> Result<Vec<SeasonDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let seasons: Vec<SqliteRow> = sqlx::query(
        "
        SELECT season_number, title
        FROM seasons
        WHERE series_id = ?
        ORDER BY season_number ASC
        ",
    )
    .bind(series_id)
    .fetch_all(&mut *tx)
    .await?;

    Ok(mapper(seasons))
}

#[instrument(skip(conn_pool))]
pub async fn query_episodes(
    conn_pool: &SqlitePool,
    series_id: i64,
    season_id: i64,
) -> Result<Vec<EpisodeDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let episodes = sqlx::query_as!(
        EpisodeDto,
        "
        SELECT e.id, e.title, e.original_title, e.plot, e.nfo_path, e.video_file_path, e.subtitle_file_path, e.thumb_image_url, e.thumb_image, e.season_number, e.episodes_number, e.runtime
        FROM episodes e
        WHERE e.season_number = ? AND e.series_id = ?
        ORDER BY e.episodes_number ASC
        ",
        season_id,
        series_id
    )
    .fetch_all(&mut *tx)
    .await?;

    Ok(episodes)
}

// TODO: after finishing the user system, query the media libraries for the current user
#[instrument(skip(conn_pool, mapper))]
pub async fn query_media_libraries(
    conn_pool: &SqlitePool,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaLibraryDto>,
) -> Result<Vec<MediaLibraryDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let raw_media_libraries: Vec<SqliteRow> =
        sqlx::query("SELECT id, name, category_id FROM media_library")
            .fetch_all(&mut *tx)
            .await?;

    Ok(mapper(raw_media_libraries))
}

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
