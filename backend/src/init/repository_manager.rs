use actix::Addr;
use anyhow::*;
use std::sync::Arc;

use crate::infrastructure::database::{
    database::Database, library::repository::LibraryRepository,
    media_item::repository::MediaRepository,
};

#[derive(Clone)]
pub struct Repositories {
    pub library: Arc<LibraryRepository>,
    pub media: Arc<MediaRepository>,
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
            library: LibraryRepository::new(self.database_addr.clone()),
            media: MediaRepository::new(self.database_addr.clone()),
        })
    }
}
