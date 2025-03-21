use actix::Addr;
use anyhow::*;
use async_trait::async_trait;
use mockall::automock;

use crate::{
    domain::media_library::model::{LibraryBrief, LibraryPoster},
    infrastructure::media_db::{
        actor::{
            DeleteLibrary, PopulateLibraryMetadata, QueryLibrary, QueryLibraryPosters, SaveLibrary,
            UpdateLibrary, ValidateCategory,
        },
        database::Database,
    },
    interfaces::http_api::controllers::api_models::{SaveLibraryPayload, UpdateLibraryPayload},
};

#[async_trait]
#[automock]
pub trait LibraryDatabase: Send + Sync {
    async fn update_library(&self, library_id: i64, payload: UpdateLibraryPayload) -> Result<()>;
    async fn query_library(&self, library_id: Option<i64>) -> Result<Vec<LibraryBrief>>;
    async fn query_library_posters(&self, library_id: i64) -> Result<Vec<LibraryPoster>>;
    async fn validate_category(&self, category_id: i64) -> Result<bool>;
    async fn save_library(&self, payload: SaveLibraryPayload) -> Result<i64>;
    async fn delete_library(&self, id: i64) -> Result<()>;
    async fn populate_library_metadata(&self, library_id: i64, item_count: usize) -> Result<()>;
}

#[derive(Clone)]
pub struct LibraryDatabaseWrapper {
    addr: Addr<Database>,
}

impl LibraryDatabaseWrapper {
    pub fn new(addr: Addr<Database>) -> Self {
        Self { addr }
    }
}

#[async_trait]
impl LibraryDatabase for LibraryDatabaseWrapper {
    async fn update_library(&self, id: i64, payload: UpdateLibraryPayload) -> Result<()> {
        self.addr
            .send(UpdateLibrary {
                id,
                name: payload.name,
                directory: payload.directory,
                category: payload.category,
            })
            .await
            .map_err(|e| anyhow!("Error updating library: {}", e))
    }

    async fn query_library(&self, library_id: Option<i64>) -> Result<Vec<LibraryBrief>> {
        self.addr
            .send(QueryLibrary { id: library_id })
            .await
            .map_err(|e| anyhow!("Error getting media libraries: {}", e))
    }

    async fn query_library_posters(&self, library_id: i64) -> Result<Vec<LibraryPoster>> {
        self.addr
            .send(QueryLibraryPosters { library_id })
            .await
            .map_err(|e| anyhow!("Error getting media libraries: {}", e))
    }

    async fn validate_category(&self, category_id: i64) -> Result<bool> {
        self.addr
            .send(ValidateCategory { category_id })
            .await
            .map_err(|e| anyhow!("Error checking if category exists: {}", e))
    }

    async fn save_library(&self, payload: SaveLibraryPayload) -> Result<i64> {
        self.addr
            .send(SaveLibrary { payload })
            .await
            .map_err(|e| anyhow!("Error creating library: {}", e))
    }

    async fn populate_library_metadata(&self, library_id: i64, item_count: usize) -> Result<()> {
        self.addr
            .send(PopulateLibraryMetadata {
                library_id,
                item_count,
            })
            .await
            .map_err(|e| anyhow!("Error populating library metadata: {}", e))
    }

    async fn delete_library(&self, id: i64) -> Result<()> {
        self.addr
            .send(DeleteLibrary { id })
            .await
            .map_err(|e| anyhow!("Error deleting library: {}", e))
    }
}
