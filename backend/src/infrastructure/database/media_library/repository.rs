use actix::Addr;
use actix_web::web::Data;
use anyhow::*;
use tracing::*;

use crate::{
    infrastructure::database::{actor::QueryMediaLibraries, database::Database},
    interfaces::dtos::MediaLibraryDto,
};

pub struct MediaLibraryRepository {
    database_addr: Data<Addr<Database>>,
}

impl MediaLibraryRepository {
    pub fn new(database_addr: Data<Addr<Database>>) -> Self {
        Self { database_addr }
    }

    #[instrument(skip(self))]
    pub async fn get_media_libraries(&self) -> Result<Vec<MediaLibraryDto>> {
        self.database_addr
            .send(QueryMediaLibraries)
            .await
            .map_err(|e| anyhow::anyhow!("Error getting media libraries: {:?}", e))
    }
}
