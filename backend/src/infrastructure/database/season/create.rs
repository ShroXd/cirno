use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::domain::season::model::Season;

#[instrument(skip(conn_pool, season))]
pub async fn save_season(
    conn_pool: &SqlitePool,
    tv_show_id: i64,
    season_number: u8,
    season: Season,
) -> Result<i64> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let season_id: i64 = sqlx::query_scalar!(
        "
        INSERT INTO seasons (series_id, season_number, title, plot, nfo_path)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT (series_id, season_number) DO UPDATE
        SET id = id
        RETURNING id;
        ",
        tv_show_id,
        season_number,
        season.title,
        season.plot,
        season.nfo_path,
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(season_id)
}
