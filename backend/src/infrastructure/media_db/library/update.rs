use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use std::sync::Arc;

use crate::{
    infrastructure::media_db::query_manager::QueryManager,
    interfaces::http_api::controllers::api_models::LibraryCategory,
};

pub async fn update_library(
    pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    id: i64,
    name: String,
    directory: String,
    category: LibraryCategory,
) -> Result<()> {
    let mut conn = pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager
        .get_query("library", "update_library_by_id")
        .await?;
    sqlx::query(&query)
        .bind(name)
        .bind(directory)
        .bind(i64::from(category))
        .bind(id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}
