use anyhow::*;
use sqlx::sqlite::SqliteRow;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::interfaces::dtos::MediaLibraryDto;

// TODO: after finishing the user system, query the media libraries for the current user
#[instrument(skip(conn_pool, mapper))]
pub async fn query_media_libraries(
    conn_pool: &SqlitePool,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaLibraryDto>,
) -> Result<Vec<MediaLibraryDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let raw_media_libraries: Vec<SqliteRow> =
        sqlx::query("SELECT id, name, category_id FROM media_library")
            .fetch_all(&mut *tx)
            .await?;

    Ok(mapper(raw_media_libraries))
}
