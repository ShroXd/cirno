use anyhow::*;
use sqlx::{Acquire, Row, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::{
    infrastructure::media_db::query_manager::QueryManager,
    interfaces::http_api::controllers::api_models::SaveLibraryPayload,
};

#[instrument(skip(conn_pool, query_manager))]
pub async fn save_library(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    library: SaveLibraryPayload,
) -> Result<i64> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager.get_query("library", "save_library").await?;

    let category_id = i64::from(library.category.clone());
    let library_id: i64 = sqlx::query(&query)
        .bind(library.name)
        .bind(library.directory)
        .bind(category_id)
        .fetch_one(&mut *tx)
        .await?
        .get(0);

    tx.commit().await?;
    Ok(library_id)
}
