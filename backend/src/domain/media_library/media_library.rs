use anyhow::Result;
use std::sync::Arc;
use tracing::*;

use crate::{
    infrastructure::database::media_library::repository::MediaLibraryRepository,
    interfaces::{
        dtos::MediaLibraryDto, http_api::controllers::api_models::SaveMediaLibraryPayload,
    },
    shared::utils::is_valid_path,
};

/// Creates a new media library after validating the directory path and checking if the category exists.
/// Returns the ID of the created media library or an error if validation fails or database operations fail.
#[instrument(skip(media_library_repository))]
pub async fn create_media_library(
    payload: SaveMediaLibraryPayload,
    media_library_repository: Arc<MediaLibraryRepository>,
) -> Result<i64> {
    debug!("Validating path");
    if !is_valid_path(&payload.directory) {
        return Err(anyhow::anyhow!("Invalid path: {}", payload.directory));
    }

    debug!("Checking if category exists");
    let category_id = i64::from(payload.category.clone());
    if !media_library_repository
        .validate_category(category_id)
        .await?
    {
        return Err(anyhow::anyhow!("Category does not exist"));
    }

    debug!("Creating media library");
    let media_library_id = media_library_repository.save_media_library(payload).await?;

    Ok(media_library_id)
}

#[instrument(skip(media_library_repository))]
pub async fn get_media_libraries(
    media_library_repository: Arc<MediaLibraryRepository>,
) -> Result<Vec<MediaLibraryDto>> {
    debug!("Getting media libraries");
    let media_libraries = media_library_repository.get_media_libraries().await?;

    Ok(media_libraries)
}

#[instrument(skip(media_library_repository))]
pub async fn get_media_library_by_id(
    id: i64,
    media_library_repository: Arc<MediaLibraryRepository>,
) -> Result<MediaLibraryDto> {
    debug!("Getting media library for id: {}", id);

    let media_library = media_library_repository.get_media_library_by_id(id).await?;

    Ok(media_library)
}

#[instrument(skip(media_library_repository))]
pub async fn delete_media_library(
    id: i64,
    media_library_repository: Arc<MediaLibraryRepository>,
) -> Result<()> {
    debug!("Deleting media library with id: {}", id);
    media_library_repository.delete_media_library(id).await?;

    Ok(())
}
