use std::sync::Arc;

use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::{
    domain::episode::model::Episode, infrastructure::database::query_manager::QueryManager,
};

#[instrument(skip(conn_pool, query_manager, episode))]
pub async fn save_episode(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    season_id: i64,
    episode: Episode,
) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager.get_query("episode", "save_episode").await?;

    sqlx::query(&query)
        .bind(season_id)
        .bind(episode.title)
        .bind(episode.original_title)
        .bind(episode.plot)
        .bind(episode.nfo_path)
        .bind(episode.video_file_path)
        .bind(episode.subtitle_file_path)
        .bind(episode.thumb_image_url)
        .bind(episode.thumb_image)
        .bind(episode.episode_number)
        .bind(episode.runtime)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    Ok(())
}
