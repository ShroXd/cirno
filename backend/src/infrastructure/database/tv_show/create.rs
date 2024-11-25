use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::services::library_parser::parsers::TVSerie;

#[instrument(skip(conn_pool, tv_show))]
pub async fn save_tv_show(
    conn_pool: &SqlitePool,
    tv_show: TVSerie,
    media_library_id: i64,
) -> Result<i64> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let id: i64 = sqlx::query_scalar!(
        "
        INSERT INTO tv_series (title, nfo_path, poster_path, fanart_path, country, year, plot, tmdb_id, imdb_id, wikidata_id, tvdb_id, media_library_id)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(title) DO UPDATE SET id = id
        RETURNING id
        ",
        tv_show.title,
        tv_show.nfo_path,
        tv_show.poster_path,
        tv_show.fanart_path,
        tv_show.country,
        tv_show.year,
        tv_show.plot,
        tv_show.tmdb_id,
        tv_show.imdb_id,
        tv_show.wikidata_id,
        tv_show.tvdb_id,
        media_library_id,
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(id)
}
