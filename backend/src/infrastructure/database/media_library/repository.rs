use actix::Addr;
use anyhow::*;
use std::sync::Arc;
use tracing::*;

use crate::{
    infrastructure::database::{actor::QueryMediaLibraries, database::Database},
    interfaces::dtos::MediaLibraryDto,
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
}
