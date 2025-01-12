use actix::Addr;
use anyhow::*;
use std::result::Result::Ok;
use std::sync::Arc;
use tracing::*;

use crate::infrastructure::database::actor::{QueryAllMedia, QueryEpisodes, QueryMediaById};
use crate::infrastructure::database::database::Database;
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
    pub async fn get_all_media(&self, library_id: i64) -> Result<Vec<MediaItemDto>> {
        debug!("Getting all media");
        let media_items = self
            .database_addr
            .send(QueryAllMedia { library_id })
            .await?;

        Ok(media_items)
    }

    #[instrument(skip(self))]
    pub async fn get_media_by_id(&self, library_id: i64, media_id: i64) -> Result<MediaItemDto> {
        debug!("Getting media for id: {}", media_id);
        let media = self
            .database_addr
            .send(QueryMediaById {
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
    pub async fn get_media_episodes(
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
            .send(QueryEpisodes {
                library_id,
                media_id,
            })
            .await?;

        Ok(episodes)
    }
}
