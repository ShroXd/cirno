use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use std::sync::Arc;

use crate::{
    domain::library::model::LibraryStatus, infrastructure::media_db::query_manager::QueryManager,
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

pub async fn populate_library_metadata(
    pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    library_id: i64,
    item_count: usize,
) -> Result<()> {
    let mut conn = pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let query = query_manager
        .get_query("library", "populate_library_metadata")
        .await?;

    let item_count = item_count as i64;
    let last_scanned = chrono::Local::now().to_rfc3339();
    let current_status = LibraryStatus::Active.to_id();
    let health_score = 100;

    sqlx::query(&query)
        .bind(item_count)
        .bind(last_scanned)
        .bind(current_status)
        .bind(health_score)
        .bind(library_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}
