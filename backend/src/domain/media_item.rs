use actix::Addr;
use anyhow::Result;
use std::sync::Arc;
use tracing::*;

use crate::{
    actors::database_actor::InsertSeries, database::database::Database,
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
