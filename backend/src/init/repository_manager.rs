use actix::Addr;
use anyhow::*;
use std::sync::Arc;

use crate::infrastructure::database::{
    database::Database, media_library::repository::MediaLibraryRepository,
};

#[derive(Clone)]
pub struct Repositories {
    pub media_library: Arc<MediaLibraryRepository>,
}

pub struct RepositoryManager {
    database_addr: Addr<Database>,
}

impl RepositoryManager {
    pub fn new(database_addr: Addr<Database>) -> Self {
        Self { database_addr }
    }

    pub fn init_repositories(&self) -> Result<Repositories> {
        Ok(Repositories {
            media_library: MediaLibraryRepository::new(self.database_addr.clone()),
        })
    }
}
