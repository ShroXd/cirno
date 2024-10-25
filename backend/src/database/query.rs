use anyhow::*;
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, SqlitePool};
use tracing::*;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct TVSeriesDTO {
    id: i64,
    title: String,
    plot: Option<String>,
    poster_path: Option<String>,
    fanart_path: Option<String>,
    country: Option<String>,
    year: Option<String>,
    genres: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct SeasonDTO {
    season_number: Option<i64>,
    season_title: Option<String>,
    episodes: Vec<EpisodeDTO>,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct EpisodeDTO {
    id: i64,
    title: Option<String>,
    original_title: Option<String>,
    plot: Option<String>,
    nfo_path: Option<String>,
    video_file_path: String,
    subtitle_file_path: Option<String>,
    thumb_image_url: Option<String>,
    thumb_image: Option<String>,
    season_number: Option<i64>,
    episodes_number: Option<i64>,
    runtime: Option<i64>,
}

#[instrument(skip(conn_pool))]
pub async fn query_series(conn_pool: &SqlitePool) -> Result<Vec<TVSeriesDTO>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let series = sqlx::query!(
        "
        select ts.id, ts.title, ts.poster_path, ts.fanart_path, ts.country, ts.year, ts.plot, group_concat(g.name, ', ') as genres
        from tv_series ts
        join tv_series_genres tsg on ts.id = tsg.series_id
        join genres g on tsg.genre_id = g.id
        group by ts.id, ts.title
        ",
    )
    .fetch_all(&mut *tx)
    .await?;

    let series: Vec<TVSeriesDTO> = series
        .into_iter()
        .map(|s| TVSeriesDTO {
            id: s.id,
            title: s.title,
            plot: s.plot,
            poster_path: s.poster_path,
            fanart_path: s.fanart_path,
            country: s.country,
            year: s.year.map(|y| y.to_string()),
            genres: s.genres.split(',').map(|s| s.to_string()).collect(),
        })
        .collect();

    Ok(series)
}

#[instrument(skip(conn_pool))]
pub async fn query_seasons_with_episodes(
    conn_pool: &SqlitePool,
    series_id: i64,
) -> Result<Vec<SeasonDTO>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

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

    let seasons: Vec<SeasonDTO> = seasons
        .into_iter()
        .map(|s| SeasonDTO {
            season_number: s.season_number,
            season_title: s.title,
            episodes: vec![],
        })
        .collect();

    let mut season_with_episodes: Vec<SeasonDTO> = vec![];
    for mut season in seasons {
        let episodes = sqlx::query_as!(
            EpisodeDTO,
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
