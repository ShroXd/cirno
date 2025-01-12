use anyhow::Result;
use std::sync::Arc;
use tracing::*;

use crate::{
    infrastructure::media_db::library::repository::LibraryRepository,
    interfaces::{dtos::LibraryDto, http_api::controllers::api_models::SaveLibraryPayload},
    shared::utils::is_valid_path,
};

/// Creates a new library after validating the directory path and checking if the category exists.
/// Returns the ID of the created library or an error if validation fails or database operations fail.
#[instrument(skip(library_repository))]
pub async fn create_library(
    payload: SaveLibraryPayload,
    library_repository: Arc<LibraryRepository>,
) -> Result<i64> {
    debug!("Validating path");
    if !is_valid_path(&payload.directory) {
        return Err(anyhow::anyhow!("Invalid path: {}", payload.directory));
    }

    debug!("Checking if category exists");
    let category_id = i64::from(payload.category.clone());
    if !library_repository.validate_category(category_id).await? {
        return Err(anyhow::anyhow!("Category does not exist"));
    }

    debug!("Creating library");
    let library_id = library_repository.save_library(payload).await?;

    Ok(library_id)
}

#[instrument(skip(library_repository))]
pub async fn get_libraries(library_repository: Arc<LibraryRepository>) -> Result<Vec<LibraryDto>> {
    debug!("Getting libraries");
    let media_libraries = library_repository.get_libraries().await?;

    Ok(media_libraries)
}

#[instrument(skip(library_repository))]
pub async fn get_library_by_id(
    id: i64,
    library_repository: Arc<LibraryRepository>,
) -> Result<LibraryDto> {
    debug!("Getting library for id: {}", id);

    let library = library_repository.get_library_by_id(id).await?;

    Ok(library)
}

#[instrument(skip(library_repository))]
pub async fn delete_library(id: i64, library_repository: Arc<LibraryRepository>) -> Result<()> {
    debug!("Deleting library with id: {}", id);
    library_repository.delete_library(id).await?;

    Ok(())
}
