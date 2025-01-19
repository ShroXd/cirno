use actix::Addr;
use anyhow::*;
use std::sync::Arc;

use crate::infrastructure::media_db::{
    database::Database,
    library::{repository::LibraryRepository, wrapper::LibraryDatabaseWrapper},
    media_item::repository::MediaRepository,
};

#[derive(Clone)]
pub struct Repositories {
    pub library: Arc<LibraryRepository<LibraryDatabaseWrapper>>,
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
        let library_database_wrapper = LibraryDatabaseWrapper::new(self.database_addr.clone());

        Ok(Repositories {
            library: LibraryRepository::new(Arc::new(library_database_wrapper)),
            media: MediaRepository::new(self.database_addr.clone()),
        })
    }
}
