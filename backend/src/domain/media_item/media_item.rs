use actix::Addr;
use actix_web::web::Data;
use anyhow::Result;
use tracing::*;

use crate::{
    infrastructure::database::{
        actor::{QueryAllMediaItems, QueryMediaItemsByMediaLibraryId},
        database::Database,
    },
    interfaces::dtos::MediaItemDto,
};

#[instrument(skip(database_addr))]
pub async fn get_media_items(
    media_library_id: Option<i64>,
    database_addr: Data<Addr<Database>>,
) -> Result<Vec<MediaItemDto>> {
    let media_items = match media_library_id {
        Some(id) => {
            debug!("Getting media items for media library id: {}", id);
            database_addr
                .send(QueryMediaItemsByMediaLibraryId(id))
                .await
                .map_err(|e| anyhow::anyhow!("Error getting media items: {:?}", e))?
        }
        None => {
            // TODO: Currently returning all media items directly when no media library is specified
            // In the future, this will return curated content like "Latest" and "You May Like"
            // sections for the main page instead of raw media items
            debug!("Getting all media items");
            database_addr
                .send(QueryAllMediaItems)
                .await
                .map_err(|e| anyhow::anyhow!("Error getting media items: {:?}", e))?
        }
    };

    Ok(media_items)
}
