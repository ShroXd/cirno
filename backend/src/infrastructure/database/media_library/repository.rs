use actix::Addr;
use anyhow::*;
use std::sync::Arc;
use tracing::*;

use crate::{
    infrastructure::database::{
        actor::{DeleteMediaLibrary, QueryMediaLibraries, SaveMediaLibrary, ValidateCategory},
        database::Database,
    },
    interfaces::{
        dtos::MediaLibraryDto, http_api::controllers::api_models::SaveMediaLibraryPayload,
    },
};

#[derive(Clone)]
pub struct MediaLibraryRepository {
    database_addr: Addr<Database>,
}

impl MediaLibraryRepository {
    pub fn new(database_addr: Addr<Database>) -> Arc<Self> {
        Arc::new(Self { database_addr })
    }

    #[instrument(skip(self))]
    pub async fn get_media_libraries(&self) -> Result<Vec<MediaLibraryDto>> {
        self.database_addr
            .send(QueryMediaLibraries)
            .await
            .map_err(|e| anyhow::anyhow!("Error getting media libraries: {:?}", e))
    }

    #[instrument(skip(self))]
    pub async fn validate_category(&self, category_id: i64) -> Result<bool> {
        self.database_addr
            .send(ValidateCategory { category_id })
            .await
            .map_err(|e| anyhow::anyhow!("Error checking if category exists: {:?}", e))
    }

    #[instrument(skip(self))]
    pub async fn save_media_library(&self, payload: SaveMediaLibraryPayload) -> Result<i64> {
        self.database_addr
            .send(SaveMediaLibrary { payload })
            .await
            .map_err(|e| anyhow::anyhow!("Error creating media library: {:?}", e))
    }

    #[instrument(skip(self))]
    pub async fn delete_media_library(&self, id: i64) -> Result<()> {
        self.database_addr
            .send(DeleteMediaLibrary { id })
            .await
            .map_err(|e| anyhow::anyhow!("Error deleting media library: {:?}", e))
    }
}
