use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::{
    interfaces::http_api::controllers::api_models::SaveMediaLibraryPayload,
    services::library_parser::parsers::{Actor, Episode, Season},
};

#[instrument(skip(conn_pool))]
pub async fn save_genre(conn_pool: &SqlitePool, series_id: i64, genre: String) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let genre_id: i64 = sqlx::query_scalar!(
        "
        INSERT INTO genres (name) VALUES (?)
        ON CONFLICT(name) DO UPDATE SET id = id
        RETURNING id
        ",
        genre,
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!(
        "INSERT OR IGNORE INTO tv_series_genres (series_id, genre_id) VALUES (?, ?)",
        series_id,
        genre_id,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

#[instrument(skip(conn_pool, actor))]
pub async fn save_actor(conn_pool: &SqlitePool, tv_show_id: i64, actor: Actor) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let actor_id: i64 = sqlx::query_scalar!(
        "
        INSERT INTO actors (name, role, thumb, profile, tmdb_id)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(name) DO UPDATE SET id = id
        RETURNING id
        ",
        actor.name,
        actor.role,
        actor.thumb,
        actor.profile,
        actor.tmdb_id,
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query!(
        "INSERT OR IGNORE INTO tv_series_actors (series_id, actor_id) VALUES (?, ?)",
        tv_show_id,
        actor_id,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

#[instrument(skip(conn_pool, season))]
pub async fn save_season(
    conn_pool: &SqlitePool,
    tv_show_id: i64,
    season_number: u8,
    season: Season,
) -> Result<i64> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let season_id: i64 = sqlx::query_scalar!(
        "
        INSERT INTO seasons (series_id, season_number, title, plot, nfo_path)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT (series_id, season_number) DO UPDATE
        SET id = id
        RETURNING id;
        ",
        tv_show_id,
        season_number,
        season.title,
        season.plot,
        season.nfo_path,
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(season_id)
}

#[instrument(skip(conn_pool, episode))]
pub async fn save_episode(
    conn_pool: &SqlitePool,
    tv_show_id: i64,
    season_id: i64,
    season_number: u8,
    episode: Episode,
) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    sqlx::query!(
        "INSERT OR IGNORE INTO episodes (series_id, season_id, title, original_title, plot, nfo_path, video_file_path, subtitle_file_path, thumb_image_url, thumb_image, season_number, episodes_number, runtime)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        tv_show_id,
        season_id,
        episode.title,
        episode.original_title,
        episode.plot,
        episode.nfo_path,
        episode.video_file_path,
        episode.subtitle_file_path,
        episode.thumb_image_url,
        episode.thumb_image,
        season_number,
        episode.episode_number,
        episode.runtime,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

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
