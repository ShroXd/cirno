use anyhow::*;
use async_trait::async_trait;
use std::path::Path;

use super::file::File;
use crate::infrastructure::file::finder_options::FinderOptions;

#[async_trait]
pub trait FileRepository: Send + Sync {
    async fn save(&self, file: &File) -> Result<()>;
    async fn find_by_path(&self, path: &str) -> Result<Option<File>>;
    async fn delete(&self, file: &File) -> Result<()>;
    async fn find_files(&self, root_dir: &Path, options: FinderOptions) -> Result<Vec<File>>;
}
