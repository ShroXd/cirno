use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::{
    domain::tv_show::model::TvShow, infrastructure::media_db::query_manager::QueryManager,
};

#[instrument(skip(conn_pool, query_manager, tv_show))]
pub async fn save_tv_show(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    tv_show: TvShow,
    library_id: i64,
) -> Result<i64> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let save_tv_show_query = query_manager.get_query("tv_show", "save_tv_show").await?;

    let tv_show_id: i64 = sqlx::query_scalar(&save_tv_show_query)
        .bind(tv_show.title)
        .bind(tv_show.original_title)
        .bind(tv_show.nfo_path)
        .bind(tv_show.poster_path)
        .bind(tv_show.fanart_path)
        .bind(tv_show.country)
        .bind(tv_show.year)
        .bind(tv_show.premiered)
        .bind(tv_show.rating)
        .bind(tv_show.runtime)
        .bind(tv_show.plot)
        .bind(tv_show.tmdb_id)
        .bind(tv_show.imdb_id)
        .bind(tv_show.wikidata_id)
        .bind(tv_show.tvdb_id)
        .fetch_one(&mut *tx)
        .await?;

    let save_library_tv_show_query = query_manager
        .get_query("tv_show", "save_library_tv_show")
        .await?;

    sqlx::query(&save_library_tv_show_query)
        .bind(library_id)
        .bind(tv_show_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(tv_show_id)
}
