use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

#[instrument(skip(conn_pool))]
pub async fn save_genre(conn_pool: &SqlitePool, series_id: i64, genre: String) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let genre_id: i64 = sqlx::query_scalar!(
        "
        INSERT INTO genres (name) VALUES (?)
        ON CONFLICT(name) DO UPDATE SET id = id
        RETURNING id
        ",
        genre,
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!(
        "INSERT OR IGNORE INTO tv_series_genres (series_id, genre_id) VALUES (?, ?)",
        series_id,
        genre_id,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
