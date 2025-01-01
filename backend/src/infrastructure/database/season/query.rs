use anyhow::*;
use sqlx::sqlite::SqliteRow;
use sqlx::{Acquire, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::{infrastructure::database::query_manager::QueryManager, interfaces::dtos::SeasonDto};

#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_seasons(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<SeasonDto>,
    series_id: i64,
) -> Result<Vec<SeasonDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager
        .get_query("season", "find_seasons_by_tv_show_id")
        .await?;

    let seasons: Vec<SqliteRow> = sqlx::query(&query)
        .bind(series_id)
        .fetch_all(&mut *tx)
        .await?;

    Ok(mapper(seasons))
}
