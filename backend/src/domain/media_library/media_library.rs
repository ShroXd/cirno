use actix::Addr;
use actix_web::web::Data;
use anyhow::Result;
use std::sync::Arc;
use tracing::*;

use crate::{
    actors::database_actor::{
        CheckCategoryExists, DeleteMediaLibrary, QueryMediaLibraries, SaveMediaLibrary,
    },
    database::database::Database,
    interfaces::{
        dtos::MediaLibraryDto, http_api::controllers::api_models::SaveMediaLibraryPayload,
    },
    shared::utils::is_valid_path,
};

/// Creates a new media library after validating the directory path and checking if the category exists.
/// Returns the ID of the created media library or an error if validation fails or database operations fail.
#[instrument(skip(database_addr))]
pub async fn create_media_library(
    payload: SaveMediaLibraryPayload,
    database_addr: Arc<Addr<Database>>,
) -> Result<i64> {
    debug!("Validating path");
    if !is_valid_path(&payload.directory) {
        return Err(anyhow::anyhow!("Invalid path: {}", payload.directory));
    }

    debug!("Checking if category exists");
    let category_id = i64::from(payload.category.clone());
    database_addr
        .send(CheckCategoryExists(category_id))
        .await
        .map_err(|e| anyhow::anyhow!("Error checking if category exists: {:?}", e))?;

    debug!("Creating media library");
    match database_addr
        .send(SaveMediaLibrary(payload, category_id))
        .await
    {
        Ok(media_library_id) => Ok(media_library_id),
        Err(e) => Err(anyhow::anyhow!("Error creating media library: {:?}", e)),
    }
}

#[instrument(skip(database_addr))]
pub async fn get_media_libraries(
    database_addr: Data<Addr<Database>>,
) -> Result<Vec<MediaLibraryDto>> {
    database_addr
        .send(QueryMediaLibraries)
        .await
        .map_err(|e| anyhow::anyhow!("Error getting media libraries: {:?}", e))
}

#[instrument(skip(database_addr))]
pub async fn delete_media_library(id: i64, database_addr: Data<Addr<Database>>) -> Result<()> {
    debug!("Deleting media library with id: {}", id);

    database_addr
        .send(DeleteMediaLibrary(id))
        .await
        .map_err(|e| anyhow::anyhow!("Error deleting media library: {:?}", e))
}
