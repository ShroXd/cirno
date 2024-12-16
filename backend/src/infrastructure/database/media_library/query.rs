use anyhow::*;
use sqlx::{sqlite::SqliteRow, Acquire, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::{
    infrastructure::database::query_manager::QueryManager, interfaces::dtos::MediaLibraryDto,
};

// TODO: after finishing the user system, query the media libraries for the current user
#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_media_libraries(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<MediaLibraryDto>,
) -> Result<Vec<MediaLibraryDto>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager
        .get_query("media_library", "get_media_libraries")
        .await?;

    let raw_media_libraries: Vec<SqliteRow> = sqlx::query(&query).fetch_all(&mut *tx).await?;

    Ok(mapper(raw_media_libraries))
}
