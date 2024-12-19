use anyhow::*;
use sqlx::{sqlite::SqliteRow, Acquire, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::{infrastructure::database::query_manager::QueryManager, interfaces::dtos::EpisodeDto};

#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_episodes(
    library_id: i64,
    media_id: i64,
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<EpisodeDto>,
) -> Result<Vec<EpisodeDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager
        .get_query("episode", "find_episodes_by_media_id")
        .await?;

    // TODO: we many need to add a intermediate table to store the library_id
    let episodes = sqlx::query(&query)
        .bind(media_id)
        .fetch_all(&mut *tx)
        .await?;

    Ok(mapper(episodes))
}
