use anyhow::*;
use sqlx::{Acquire, Row, SqlitePool};
use std::sync::Arc;
use tracing::*;

use crate::{
    domain::media_actor::model::MediaActor, infrastructure::database::query_manager::QueryManager,
};

#[instrument(skip(conn_pool, query_manager, actor))]
pub async fn save_actor(
    conn_pool: &SqlitePool,
    query_manager: Arc<dyn QueryManager>,
    tv_show_id: i64,
    actor: MediaActor,
) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let save_actor_query = query_manager.get_query("actor", "save_actor").await?;
    let actor_id: i64 = sqlx::query(&save_actor_query)
        .bind(actor.name)
        .bind(actor.role)
        .bind(actor.thumb)
        .bind(actor.profile)
        .bind(actor.tmdb_id)
        .fetch_one(&mut *tx)
        .await?
        .get(0);

    let save_actor_to_tv_show_query = query_manager
        .get_query("actor", "save_actor_to_tv_show")
        .await?;

    sqlx::query(&save_actor_to_tv_show_query)
        .bind(tv_show_id)
        .bind(actor_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}
