use anyhow::*;
use std::result::Result::Ok;
use std::sync::Arc;
use tracing::*;

use crate::interfaces::{
    dtos::LibraryDto,
    http_api::controllers::api_models::{SaveLibraryPayload, UpdateLibraryPayload},
};

use super::wrapper::LibraryDatabase;

#[derive(Clone)]
pub struct LibraryRepository<D: LibraryDatabase> {
    database: Arc<D>,
}

impl<D: LibraryDatabase> LibraryRepository<D> {
    pub fn new(database: Arc<D>) -> Arc<Self> {
        Arc::new(Self { database })
    }

    #[instrument(skip(self))]
    pub async fn update_library(&self, id: i64, payload: UpdateLibraryPayload) -> Result<()> {
        self.database.update_library(id, payload).await
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
        let media_library_briefs = self.database.query_library(id).await?;

        let futures: Vec<_> = media_library_briefs
            .into_iter()
            .map(|media_library_brief| async move {
                let media_library_posters = match self
                    .database
                    .query_library_posters(media_library_brief.id)
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
        self.database.validate_category(category_id).await
    }

    #[instrument(skip(self))]
    pub async fn save_library(&self, payload: SaveLibraryPayload) -> Result<i64> {
        self.database.save_library(payload).await
    }

    #[instrument(skip(self))]
    pub async fn delete_library(&self, id: i64) -> Result<()> {
        self.database.delete_library(id).await
    }
}

#[cfg(test)]
mod tests {
    use actix::{Actor, Context};

    struct MockDatabase;

    impl Actor for MockDatabase {
        type Context = Context<Self>;
    }
}
