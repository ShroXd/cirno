use anyhow::*;
use std::sync::Arc;

use crate::{
    domain::file::file_repository::FileRepository,
    infrastructure::{
        event_bus::event_bus::EventBus,
        file::finder_options::{by_extension, FinderOptions},
    },
};

#[derive(Clone)]
pub struct FileService {
    file_repository: Arc<dyn FileRepository>,
    event_bus: Arc<EventBus>,
}

impl FileService {
    pub fn new(file_repository: Arc<dyn FileRepository>, event_bus: Arc<EventBus>) -> Self {
        Self {
            file_repository,
            event_bus,
        }
    }

    pub async fn delete_files_in_folder(
        &self,
        folder_path: &str,
        options: FinderOptions,
    ) -> Result<()> {
        let files = self
            .file_repository
            .find_files(folder_path.as_ref(), options)
            .await?;

        for file in files {
            self.file_repository.delete(&file).await?;
        }
        Ok(())
    }
}
