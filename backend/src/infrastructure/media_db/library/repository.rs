use anyhow::*;
use std::result::Result::Ok;
use std::sync::Arc;
use tracing::*;

use super::wrapper::LibraryDatabase;
use crate::interfaces::{
    dtos::LibraryDto,
    http_api::controllers::api_models::{SaveLibraryPayload, UpdateLibraryPayload},
};

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
                    item_count: media_library_brief.item_count,
                    last_scanned: media_library_brief.last_scanned,
                    current_status: media_library_brief.current_status,
                    auto_scan: media_library_brief.auto_scan,
                    error: media_library_brief.error,
                    storage_used: media_library_brief.storage_used,
                    health_score: media_library_brief.health_score,
                    created_at: media_library_brief.created_at,
                    updated_at: media_library_brief.updated_at,
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
    pub async fn populate_library_metadata(
        &self,
        library_id: i64,
        item_count: usize,
    ) -> Result<()> {
        self.database
            .populate_library_metadata(library_id, item_count)
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_library(&self, id: i64) -> Result<()> {
        self.database.delete_library(id).await
    }
}

#[cfg(test)]
mod tests {
    use futures::future::ready;
    use mockall::predicate::{self, *};
    use std::result::Result::Ok;

    use super::*;
    use crate::{
        domain::{
            library::model::LibraryStatus,
            media_library::model::{LibraryBrief, LibraryPoster},
        },
        infrastructure::media_db::library::wrapper::MockLibraryDatabase,
        interfaces::http_api::controllers::api_models::LibraryCategory,
    };

    #[tokio::test]
    async fn test_update_library_success() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_update_library()
            .with(eq(1), predicate::always())
            .returning(|_, _| Box::pin(ready(Ok(()))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository
            .update_library(1, UpdateLibraryPayload::default())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_library_fail() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_update_library()
            .with(eq(1), predicate::always())
            .returning(|_, _| Box::pin(ready(Err(anyhow::anyhow!("Error updating library")))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository
            .update_library(1, UpdateLibraryPayload::default())
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Error updating library");
    }

    #[tokio::test]
    async fn test_library_internal_success() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_query_library()
            .with(eq(Some(1)))
            .returning(|_| {
                Box::pin(ready(Ok(vec![LibraryBrief {
                    id: 1,
                    name: "test".to_string(),
                    category: LibraryCategory::Movie,
                    directory: "test".to_string(),
                    item_count: 1,
                    last_scanned: Some("2021-01-01".to_string()),
                    current_status: LibraryStatus::Active,
                    auto_scan: true,
                    error: None,
                    storage_used: 1,
                    health_score: 1,
                    created_at: "2021-01-01".to_string(),
                    updated_at: "2021-01-01".to_string(),
                }])))
            });

        mock_database
            .expect_query_library_posters()
            .with(eq(1))
            .returning(|_| Box::pin(ready(Ok(vec![LibraryPoster::default()]))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository.get_library_internal(Some(1)).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![LibraryDto {
                id: 1,
                name: "test".to_string(),
                category: LibraryCategory::Movie,
                directory: "test".to_string(),
                posters: vec![LibraryPoster::default()],
                item_count: 1,
                last_scanned: Some("2021-01-01".to_string()),
                current_status: LibraryStatus::Active,
                auto_scan: true,
                error: None,
                storage_used: 1,
                health_score: 1,
                created_at: "2021-01-01".to_string(),
                updated_at: "2021-01-01".to_string(),
            }]
        );
    }

    #[tokio::test]
    async fn test_library_internal_empty_success() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_query_library()
            .with(eq(Some(1)))
            .returning(|_| Box::pin(ready(Ok(vec![]))));

        mock_database
            .expect_query_library_posters()
            .with(eq(1))
            .returning(|_| Box::pin(ready(Ok(vec![]))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository.get_library_internal(Some(1)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![]);
    }

    #[tokio::test]
    async fn test_library_internal_fail() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_query_library()
            .with(eq(Some(1)))
            .returning(|_| {
                Box::pin(ready(Ok(vec![LibraryBrief {
                    id: 1,
                    name: "test".to_string(),
                    category: LibraryCategory::Movie,
                    directory: "test".to_string(),
                    item_count: 1,
                    last_scanned: Some("2021-01-01".to_string()),
                    current_status: LibraryStatus::Active,
                    auto_scan: true,
                    error: None,
                    storage_used: 1,
                    health_score: 1,
                    created_at: "2021-01-01".to_string(),
                    updated_at: "2021-01-01".to_string(),
                }])))
            });

        mock_database
            .expect_query_library_posters()
            .with(eq(1))
            .returning(|_| Box::pin(ready(Err(anyhow::anyhow!("Error getting library posters")))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository.get_library_internal(Some(1)).await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![LibraryDto {
                id: 1,
                name: "test".to_string(),
                category: LibraryCategory::Movie,
                directory: "test".to_string(),
                posters: vec![],
                item_count: 1,
                last_scanned: Some("2021-01-01".to_string()),
                current_status: LibraryStatus::Active,
                auto_scan: true,
                error: None,
                storage_used: 1,
                health_score: 1,
                created_at: "2021-01-01".to_string(),
                updated_at: "2021-01-01".to_string(),
            }]
        );
    }

    #[tokio::test]
    async fn test_validate_category_success() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_validate_category()
            .with(eq(1))
            .returning(|_| Box::pin(ready(Ok(true))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository.validate_category(1).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_validate_category_fail() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_validate_category()
            .with(eq(1))
            .returning(|_| Box::pin(ready(Err(anyhow::anyhow!("Error validating category")))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository.validate_category(1).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Error validating category");
    }

    #[tokio::test]
    async fn test_save_library_success() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_save_library()
            .with(eq(SaveLibraryPayload::default()))
            .returning(|_| Box::pin(ready(Ok(1))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository.save_library(SaveLibraryPayload::default()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_save_library_fail() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_save_library()
            .with(eq(SaveLibraryPayload::default()))
            .returning(|_| Box::pin(ready(Err(anyhow::anyhow!("Error saving library")))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository.save_library(SaveLibraryPayload::default()).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Error saving library");
    }

    #[tokio::test]
    async fn test_delete_library_success() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_delete_library()
            .with(eq(1))
            .returning(|_| Box::pin(ready(Ok(()))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository.delete_library(1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_library_fail() {
        let mut mock_database = MockLibraryDatabase::new();
        mock_database
            .expect_delete_library()
            .with(eq(1))
            .returning(|_| Box::pin(ready(Err(anyhow::anyhow!("Error deleting library")))));

        let repository = LibraryRepository::new(Arc::new(mock_database));
        let result = repository.delete_library(1).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Error deleting library");
    }
}
