use anyhow::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use sqlx::sqlite::SqliteRow;
use sqlx::{Acquire, QueryBuilder, Row, SqlitePool};
use tracing::*;

use crate::interfaces::dtos::{EpisodeDto, MediaItemDto, MediaLibraryDto, SeasonDto};

#[instrument(skip(conn_pool))]
pub async fn query_series(
    conn_pool: &SqlitePool,
    media_library_id: i64,
) -> Result<Vec<MediaItemDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let mut query_builder = QueryBuilder::new(
        "
        SELECT ts.id, ts.title, ts.poster_path, ts.fanart_path, ts.country, ts.year, ts.plot,
               group_concat(g.name, ', ') AS genres
        FROM tv_series ts
        JOIN tv_series_genres tsg ON ts.id = tsg.series_id
        JOIN genres g ON tsg.genre_id = g.id
        ",
    );
    // query_builder.push("WHERE ts.media_library_id = ");
    // query_builder.push_bind(media_library_id);
    query_builder.push("GROUP BY ts.id, ts.title");

    let query = query_builder.build();
    let series = query.fetch_all(&mut *tx).await?;

    let series: Vec<MediaItemDto> = series
        .par_iter()
        .map(|s| MediaItemDto {
            id: s.get::<i64, _>("id"),
            title: s.get::<String, _>("title"),
            plot: s.get::<Option<String>, _>("plot"),
            poster_path: s.get::<Option<String>, _>("poster_path"),
            fanart_path: s.get::<Option<String>, _>("fanart_path"),
            country: s.get::<Option<String>, _>("country"),
            year: s.get::<Option<i64>, _>("year").map(|y| y.to_string()),
            genres: s
                .get::<String, _>("genres")
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        })
        .collect();

    Ok(series)
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
