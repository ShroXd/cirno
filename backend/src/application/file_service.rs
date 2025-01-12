use anyhow::*;
use std::sync::Arc;

use crate::{
    domain::file::file_repository::FileRepository,
    infrastructure::file::finder_options::FinderOptions,
};

#[derive(Clone)]
pub struct FileService {
    file_repository: Arc<dyn FileRepository>,
}

impl FileService {
    pub fn new(file_repository: Arc<dyn FileRepository>) -> Self {
        Self { file_repository }
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
