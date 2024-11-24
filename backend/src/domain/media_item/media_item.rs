use actix::Addr;
use actix_web::web::Data;
use anyhow::Result;
use std::sync::Arc;
use tracing::*;

use crate::{
    actors::database_actor::{GetMediaItems, InsertSeries},
    database::database::Database,
    interfaces::dtos::MediaItemDto,
    services::library_parser::parsers::TVSerie,
};

#[instrument(skip(media_item, database_addr))]
pub async fn insert_media_item(
    media_library_id: i64,
    media_item: TVSerie,
    database_addr: Arc<Addr<Database>>,
) -> Result<()> {
    debug!("Inserting media item: {:?}", media_item.title);

    // TODO: handle complex insertions in the domain layer
    database_addr
        .send(InsertSeries(media_item, media_library_id))
        .await
        .map_err(|e| anyhow::anyhow!("Error inserting media item: {:?}", e))?;

    Ok(())
}

#[instrument(skip(database_addr))]
pub async fn get_media_items(
    media_library_id: i64,
    database_addr: Data<Addr<Database>>,
) -> Result<Vec<MediaItemDto>> {
    debug!(
        "Getting media items for media library id: {}",
        media_library_id
    );

    let media_items = database_addr
        .send(GetMediaItems(media_library_id))
        .await
        .map_err(|e| anyhow::anyhow!("Error getting media items: {:?}", e))?;

    Ok(media_items)
}
