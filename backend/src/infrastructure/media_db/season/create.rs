use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::{domain::season::model::Season, infrastructure::media_db::query_manager::QueryManager};

#[instrument(skip(conn_pool, query_manager, season))]
pub async fn save_season(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    tv_show_id: i64,
    season_number: u8,
    season: Season,
) -> Result<i64> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager.get_query("season", "save_season").await?;

    let season_id: i64 = sqlx::query_scalar(&query)
        .bind(tv_show_id)
        .bind(season_number)
        .bind(season.title)
        .bind(season.plot)
        .bind(season.nfo_path)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(season_id)
}
