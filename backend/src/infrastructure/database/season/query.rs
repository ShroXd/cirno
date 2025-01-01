use anyhow::*;
use sqlx::sqlite::SqliteRow;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::interfaces::dtos::SeasonDto;

#[instrument(skip(conn_pool, mapper))]
pub async fn query_seasons(
    conn_pool: &SqlitePool,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<SeasonDto>,
    series_id: i64,
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
