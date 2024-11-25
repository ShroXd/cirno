use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::interfaces::dtos::EpisodeDto;

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
