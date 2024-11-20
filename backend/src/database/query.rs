use anyhow::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use sqlx::{Acquire, QueryBuilder, Row, SqlitePool};
use tracing::*;

use crate::application::dtos::{EpisodeDto, MediaItemDto, MediaLibraryDto, SeasonDto};
use crate::application::http_api::controllers::api_models::MediaLibraryCategory;

#[instrument(skip(conn_pool))]
pub async fn query_series(
    conn_pool: &SqlitePool,
    media_library_id: Option<i64>,
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
    if let Some(media_library_id) = media_library_id {
        query_builder.push("WHERE ts.media_library_id = ");
        query_builder.push_bind(media_library_id);
    }
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

#[instrument(skip(conn_pool))]
pub async fn query_seasons_with_episodes(
    conn_pool: &SqlitePool,
    series_id: i64,
) -> Result<Vec<SeasonDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    // TODO: use uuid for series_id instead of i64

    let seasons = sqlx::query!(
        "
        SELECT season_number, title
        FROM seasons
        WHERE series_id = ?
        ORDER BY season_number ASC
        ",
        series_id
    )
    .fetch_all(&mut *tx)
    .await?;

    let seasons: Vec<SeasonDto> = seasons
        .into_iter()
        .map(|s| SeasonDto {
            season_number: s.season_number,
            season_title: s.title,
            episodes: vec![],
        })
        .collect();

    let mut season_with_episodes: Vec<SeasonDto> = vec![];
    for mut season in seasons {
        let episodes = sqlx::query_as!(
            EpisodeDto,
            "
            SELECT e.id, e.title, e.original_title, e.plot, e.nfo_path, e.video_file_path, e.subtitle_file_path, e.thumb_image_url, e.thumb_image, e.season_number, e.episodes_number, e.runtime
            FROM episodes e
            WHERE e.season_number = ? AND e.series_id = ?
            ORDER BY e.episodes_number ASC
            ",
            season.season_number,
            series_id
        )
        .fetch_all(&mut *tx)
        .await?;

        season.episodes = episodes;
        season_with_episodes.push(season);
    }

    Ok(season_with_episodes)
}

// TODO: after finishing the user system, query the media libraries for the current user
#[instrument(skip(conn_pool))]
pub async fn query_media_libraries(conn_pool: &SqlitePool) -> Result<Vec<MediaLibraryDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let raw_media_libraries = sqlx::query!("SELECT id, name, category_id FROM media_library",)
        .fetch_all(&mut *tx)
        .await?;

    let media_libraries: Vec<MediaLibraryDto> = raw_media_libraries
        .into_iter()
        .map(|ml| MediaLibraryDto {
            id: ml.id,
            name: ml.name,
            // TODO: consider if this is the best way to handle this
            category: MediaLibraryCategory::try_from(ml.category_id)
                .unwrap_or(MediaLibraryCategory::Movie),
        })
        .collect();
    debug!("Found {} media libraries", media_libraries.len());

    Ok(media_libraries)
}
