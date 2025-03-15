use actix::Addr;
use anyhow::*;
use std::result::Result::Ok;
use std::sync::Arc;
use tracing::*;

use crate::infrastructure::media_db::actor::{
    QueryLibraryMedia, QueryLibraryMediaEpisodes, QueryLibraryMedias, QueryMediaById,
    QueryMediaEpisodes,
};
use crate::infrastructure::media_db::database::Database;
use crate::interfaces::dtos::{EpisodeDto, MediaItemDto};

#[derive(Clone)]
pub struct MediaRepository {
    database_addr: Addr<Database>,
}

impl MediaRepository {
    pub fn new(database_addr: Addr<Database>) -> Arc<Self> {
        Arc::new(Self { database_addr })
    }

    #[instrument(skip(self))]
    pub async fn get_library_medias(&self, library_id: i64) -> Result<Vec<MediaItemDto>> {
        debug!("Getting all media");
        let media_items = self
            .database_addr
            .send(QueryLibraryMedias { library_id })
            .await?;

        Ok(media_items)
    }

    #[instrument(skip(self))]
    pub async fn get_library_media(&self, library_id: i64, media_id: i64) -> Result<MediaItemDto> {
        debug!("Getting media for id: {}", media_id);
        let media = self
            .database_addr
            .send(QueryLibraryMedia {
                library_id,
                media_id,
            })
            .await?;

        if media.is_empty() {
            return Err(anyhow::anyhow!("Media item not found"));
        }

        Ok(media.first().unwrap().clone())
    }

    #[instrument(skip(self))]
    pub async fn get_library_media_episodes(
        &self,
        library_id: i64,
        media_id: i64,
    ) -> Result<Vec<EpisodeDto>> {
        debug!(
            "Getting media episodes for id: {} in library: {}",
            media_id, library_id
        );

        let episodes = self
            .database_addr
            .send(QueryLibraryMediaEpisodes {
                library_id,
                media_id,
            })
            .await?;

        Ok(episodes)
    }

    #[instrument(skip(self))]
    pub async fn get_media_by_id(&self, media_id: i64) -> Result<MediaItemDto> {
        debug!("Getting media for id: {}", media_id);
        match self.database_addr.send(QueryMediaById { media_id }).await? {
            Some(media) => Ok(media),
            None => Err(anyhow::anyhow!("Media item not found")),
        }
    }

    #[instrument(skip(self))]
    pub async fn get_media_episodes(&self, media_id: i64) -> Result<Vec<EpisodeDto>> {
        debug!("Getting media episodes for id: {}", media_id);

        let episodes = self
            .database_addr
            .send(QueryMediaEpisodes { media_id })
            .await?;

        Ok(episodes)
    }
}
