use anyhow::*;
use sqlx::{sqlite::SqliteRow, Acquire, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::{
    domain::media_library::model::{LibraryBrief, LibraryPoster},
    infrastructure::media_db::query_manager::QueryManager,
};

// TODO: after finishing the user system, query the media libraries for the current user
#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_library(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<LibraryBrief>,
    id: Option<i64>,
) -> Result<Vec<LibraryBrief>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let raw_library = match id {
        Some(id) => {
            let query = query_manager
                .get_query("library", "find_library_by_id")
                .await?;
            sqlx::query(&query).bind(id).fetch_all(&mut *tx).await?
        }
        None => {
            let query = query_manager
                .get_query("library", "find_all_libraries")
                .await?;
            sqlx::query(&query).fetch_all(&mut *tx).await?
        }
    };

    Ok(mapper(raw_library))
}

#[instrument(skip(conn_pool, query_manager, mapper))]
pub async fn query_library_posters(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    mapper: impl Fn(Vec<SqliteRow>) -> Vec<LibraryPoster>,
    library_id: i64,
) -> Result<Vec<LibraryPoster>> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    debug!("Querying library posters for {}", library_id);

    let query = query_manager
        .get_query("library", "get_library_posters")
        .await?;

    let raw_library_posters: Vec<SqliteRow> = sqlx::query(&query)
        .bind(library_id)
        .fetch_all(&mut *tx)
        .await?;

    Ok(mapper(raw_library_posters))
}
