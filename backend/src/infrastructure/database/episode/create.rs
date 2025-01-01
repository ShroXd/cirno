use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::domain::episode::model::Episode;

#[instrument(skip(conn_pool, episode))]
pub async fn save_episode(
    conn_pool: &SqlitePool,
    _tv_show_id: i64,
    season_id: i64,
    episode: Episode,
) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    sqlx::query!(
        r#"INSERT OR IGNORE INTO episodes (season_id, title, original_title, plot, nfo_path, video_file_path, subtitle_file_path, thumb_image_url, thumb_image, episode_number, runtime)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        season_id,
        episode.title,
        episode.original_title,
        episode.plot,
        episode.nfo_path,
        episode.video_file_path,
        episode.subtitle_file_path,
        episode.thumb_image_url,
        episode.thumb_image,
        episode.episode_number,
        episode.runtime,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
