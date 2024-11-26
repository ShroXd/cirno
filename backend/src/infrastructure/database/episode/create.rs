use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::domain::episode::model::Episode;

#[instrument(skip(conn_pool, episode))]
pub async fn save_episode(
    conn_pool: &SqlitePool,
    tv_show_id: i64,
    season_id: i64,
    season_number: u8,
    episode: Episode,
) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    sqlx::query!(
        "INSERT OR IGNORE INTO episodes (series_id, season_id, title, original_title, plot, nfo_path, video_file_path, subtitle_file_path, thumb_image_url, thumb_image, season_number, episodes_number, runtime)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        tv_show_id,
        season_id,
        episode.title,
        episode.original_title,
        episode.plot,
        episode.nfo_path,
        episode.video_file_path,
        episode.subtitle_file_path,
        episode.thumb_image_url,
        episode.thumb_image,
        season_number,
        episode.episode_number,
        episode.runtime,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
