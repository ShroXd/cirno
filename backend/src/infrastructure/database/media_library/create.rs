use anyhow::*;
use sqlx::{Acquire, Row, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::{
    infrastructure::database::query_manager::QueryManager,
    interfaces::http_api::controllers::api_models::SaveMediaLibraryPayload,
};

#[instrument(skip(conn_pool, query_manager))]
pub async fn save_media_library(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    media_library: SaveMediaLibraryPayload,
) -> Result<i64> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager
        .get_query("media_library", "save_media_library")
        .await?;

    let category_id = i64::from(media_library.category.clone());
    let media_library_id: i64 = sqlx::query(&query)
        .bind(media_library.name)
        .bind(media_library.directory)
        .bind(category_id)
        .fetch_one(&mut *tx)
        .await?
        .get(0);

    tx.commit().await?;
    Ok(media_library_id)
}
