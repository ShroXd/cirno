use actix::Addr;
use anyhow::*;
use std::result::Result::Ok;
use std::sync::Arc;
use tracing::*;

use crate::{
    infrastructure::database::{
        actor::{
            DeleteMediaLibrary, QueryMediaLibrary, QueryMediaLibraryPosters, SaveMediaLibrary,
            ValidateCategory,
        },
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
        self.get_media_library_internal(None).await
    }

    #[instrument(skip(self))]
    pub async fn get_media_library_by_id(&self, id: i64) -> Result<MediaLibraryDto> {
        let results = self.get_media_library_internal(Some(id)).await?;

        if results.is_empty() {
            return Err(anyhow::anyhow!("Media library not found"));
        }

        Ok(results.into_iter().next().unwrap())
    }

    #[instrument(skip(self))]
    pub async fn get_media_library_internal(
        &self,
        id: Option<i64>,
    ) -> Result<Vec<MediaLibraryDto>> {
        let media_library_briefs = self
            .database_addr
            .send(QueryMediaLibrary { id })
            .await
            .map_err(|e| anyhow::anyhow!("Error getting media libraries: {:?}", e))?;

        let futures: Vec<_> = media_library_briefs
            .into_iter()
            .map(|media_library_brief| async move {
                let media_library_posters = match self
                    .database_addr
                    .send(QueryMediaLibraryPosters {
                        media_library_id: media_library_brief.id,
                    })
                    .await
                {
                    Ok(posters) => posters,
                    Err(e) => {
                        error!(
                            "Error getting posters for media library {}: {:?}",
                            media_library_brief.id, e
                        );
                        vec![]
                    }
                };

                MediaLibraryDto {
                    id: media_library_brief.id,
                    name: media_library_brief.name,
                    category: media_library_brief.category,
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
