use actix::Addr;
use anyhow::*;
use std::result::Result::Ok;
use std::sync::Arc;
use tracing::*;

use crate::{
    infrastructure::database::{
        actor::{
            DeleteLibrary, QueryLibrary, QueryLibraryPosters, SaveLibrary, UpdateLibrary,
            ValidateCategory,
        },
        database::Database,
    },
    interfaces::{
        dtos::LibraryDto,
        http_api::controllers::api_models::{SaveLibraryPayload, UpdateLibraryPayload},
    },
};

#[derive(Clone)]
pub struct LibraryRepository {
    database_addr: Addr<Database>,
}

impl LibraryRepository {
    pub fn new(database_addr: Addr<Database>) -> Arc<Self> {
        Arc::new(Self { database_addr })
    }

    #[instrument(skip(self))]
    pub async fn update_library(&self, id: i64, payload: UpdateLibraryPayload) -> Result<()> {
        self.database_addr
            .send(UpdateLibrary {
                id,
                name: payload.name,
                directory: payload.directory,
                category: payload.category,
            })
            .await
            .map_err(|e| anyhow::anyhow!("Error updating library: {:?}", e))
    }

    #[instrument(skip(self))]
    pub async fn get_libraries(&self) -> Result<Vec<LibraryDto>> {
        self.get_library_internal(None).await
    }

    #[instrument(skip(self))]
    pub async fn get_library_by_id(&self, id: i64) -> Result<LibraryDto> {
        let results = self.get_library_internal(Some(id)).await?;

        if results.is_empty() {
            return Err(anyhow::anyhow!("Media library not found"));
        }

        Ok(results.into_iter().next().unwrap())
    }

    #[instrument(skip(self))]
    pub async fn get_library_internal(&self, id: Option<i64>) -> Result<Vec<LibraryDto>> {
        let media_library_briefs = self
            .database_addr
            .send(QueryLibrary { id })
            .await
            .map_err(|e| anyhow::anyhow!("Error getting media libraries: {:?}", e))?;

        let futures: Vec<_> = media_library_briefs
            .into_iter()
            .map(|media_library_brief| async move {
                let media_library_posters = match self
                    .database_addr
                    .send(QueryLibraryPosters {
                        library_id: media_library_brief.id,
                    })
                    .await
                {
                    Ok(posters) => posters,
                    Err(e) => {
                        error!(
                            "Error getting posters for library {}: {:?}",
                            media_library_brief.id, e
                        );
                        vec![]
                    }
                };

                LibraryDto {
                    id: media_library_brief.id,
                    name: media_library_brief.name,
                    category: media_library_brief.category,
                    directory: media_library_brief.directory,
                    posters: media_library_posters,
                }
            })
            .collect();

        let res = futures::future::join_all(futures).await;

        Ok(res)
    }

    #[instrument(skip(self))]
    pub async fn validate_category(&self, category_id: i64) -> Result<bool> {
        self.database_addr
            .send(ValidateCategory { category_id })
            .await
            .map_err(|e| anyhow::anyhow!("Error checking if category exists: {:?}", e))
    }

    #[instrument(skip(self))]
    pub async fn save_library(&self, payload: SaveLibraryPayload) -> Result<i64> {
        self.database_addr
            .send(SaveLibrary { payload })
            .await
            .map_err(|e| anyhow::anyhow!("Error creating library: {:?}", e))
    }

    #[instrument(skip(self))]
    pub async fn delete_library(&self, id: i64) -> Result<()> {
        self.database_addr
            .send(DeleteLibrary { id })
            .await
            .map_err(|e| anyhow::anyhow!("Error deleting library: {:?}", e))
    }
}
