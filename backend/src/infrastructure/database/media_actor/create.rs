use anyhow::*;
use sqlx::{Acquire, SqlitePool};
use tracing::*;

use crate::services::library_parser::parsers::{Actor, Episode};

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
