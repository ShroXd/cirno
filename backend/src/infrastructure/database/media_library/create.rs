use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::interfaces::http_api::controllers::api_models::SaveMediaLibraryPayload;

#[instrument(skip(conn_pool))]
pub async fn save_media_library(
    conn_pool: &SqlitePool,
    media_library: SaveMediaLibraryPayload,
    category_id: i64,
) -> Result<i64> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let media_library_id: i64 = sqlx::query_scalar!(
        "INSERT OR IGNORE INTO media_library (name, directory, category_id) VALUES (?, ?, ?) RETURNING id",
        media_library.name,
        media_library.directory,
        category_id,
    )
        .fetch_one(&mut *tx)
        .await?
        .ok_or(anyhow::anyhow!("Failed to get media library ID"))?;

    tx.commit().await?;
    Ok(media_library_id)
}
