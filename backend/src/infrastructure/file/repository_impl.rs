use anyhow::*;
use async_trait::async_trait;
use std::path::Path;
use std::result::Result::Ok;
use tokio::fs::{self, remove_file};
use tracing::*;
use walkdir::{DirEntry, WalkDir};

use super::finder_options::FinderOptions;
use crate::domain::file::{file::File, file_repository::FileRepository};

pub struct FileRepositoryImpl {}

#[async_trait]
impl FileRepository for FileRepositoryImpl {
    async fn save(&self, _file: &File) -> Result<()> {
        Ok(())
    }

    async fn find_by_path(&self, _path: &str) -> Result<Option<File>> {
        Ok(None)
    }

    #[instrument(skip(self, file))]
    async fn delete(&self, file: &File) -> Result<()> {
        debug!("Attempting to delete file: {:?}", file.path);

        if !file.path.exists() {
            warn!("File does not exist: {:?}", file.path);
            return Ok(());
        }

        if !file.path.is_file() {
            warn!("File is not a file: {:?}", file.path);
            return Ok(());
        }

        match fs::metadata(&file.path).await {
            Ok(metadata) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if metadata.permissions().mode() & 0o002 != 0 {
                        warn!("File is not writable: {:?}", file.path);
                        return Ok(());
                    }
                }
                #[cfg(windows)]
                {
                    use std::os::windows::fs::PermissionsExt;
                    if metadata.permissions().mode() & 0o002 != 0 {
                        warn!("File is not writable: {:?}", file.path);
                        return Ok(());
                    }
                }
            }
            Err(e) => return Err(anyhow::anyhow!("Failed to delete file: {:?}", e)),
        }

        remove_file(&file.path)
            .await
            .map(|_| {
                debug!("File deleted successfully: {:?}", file.path);
            })
            .map_err(|e| anyhow::anyhow!("Failed to delete file: {:?}", e))
    }

    #[instrument(skip(self, root_dir, options))]
    async fn find_files(&self, root_dir: &Path, options: FinderOptions) -> Result<Vec<File>> {
        debug!("Finding files in: {:?}", root_dir);

        if !root_dir.exists() {
            warn!("Root directory does not exist: {:?}", root_dir);
            return Ok(vec![]);
        }

        if !root_dir.is_dir() {
            warn!("Root directory is not a directory: {:?}", root_dir);
            return Ok(vec![]);
        }

        let mut walker = WalkDir::new(root_dir);
        if !options.recursive {
            walker = walker.max_depth(1);
        }

        let is_hidden = |entry: &DirEntry| {
            entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        };

        let files: Vec<File> = walker
            .into_iter()
            .filter_entry(|e| {
                options.include_hidden || {
                    debug!("Checking if file is hidden: {:?}", e.path());
                    let flag = is_hidden(e);
                    !flag
                }
            })
            .filter_map(|e| {
                debug!("Filtering file: {:?}", e);
                e.ok()
            })
            .filter(|e| {
                debug!("Checking if file is a file: {:?}", e.path());
                e.file_type().is_file()
            })
            .map(|e| File::new(e.path().to_path_buf()))
            .filter(|file| {
                options
                    .filters
                    .as_ref()
                    .map(|filter| filter(&file.path))
                    .unwrap_or(true)
            })
            .collect();

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::Builder;
    use tokio::fs::File;

    use super::*;

    #[tokio::test]
    async fn test_find_files() -> Result<()> {
        let temp_dir = Builder::new().prefix("test-").tempdir()?;
        let temp_path = temp_dir.path();

        let file1_path = temp_path.join("test1.txt");
        let file2_path = temp_path.join("test2.txt");
        let hidden_file_path = temp_path.join(".hidden.txt");

        let _file1 = File::create(&file1_path).await?;
        let _file2 = File::create(&file2_path).await?;
        let _hidden = File::create(&hidden_file_path).await?;

        let repo = FileRepositoryImpl {};

        let options = FinderOptions {
            recursive: false,
            include_hidden: false,
            filters: None,
        };
        let files = repo.find_files(temp_path, options).await?;
        assert_eq!(files.len(), 2);

        let options_with_hidden = FinderOptions {
            recursive: false,
            include_hidden: true,
            filters: None,
        };
        let files_with_hidden = repo.find_files(temp_path, options_with_hidden).await?;
        assert_eq!(files_with_hidden.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_find_files_with_filter() -> Result<()> {
        let temp_dir = Builder::new().prefix("test-").tempdir()?;
        let file1_path = temp_dir.path().join("test1.txt");
        let file2_path = temp_dir.path().join("test2.log");

        let _file1 = File::create(&file1_path).await?;
        let _file2 = File::create(&file2_path).await?;

        let repo = FileRepositoryImpl {};

        let options = FinderOptions {
            recursive: false,
            include_hidden: false,
            filters: Some(Box::new(|path: &Path| {
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "txt")
                    .unwrap_or(false)
            })),
        };

        let files = repo.find_files(temp_dir.path(), options).await?;
        assert_eq!(files.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_file() {
        let temp_dir = Builder::new().prefix("test-").tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let _file = File::create(&file_path).await.unwrap();

        let repo = FileRepositoryImpl {};

        // Verify file exists
        let files = repo
            .find_files(temp_dir.path(), FinderOptions::default())
            .await
            .unwrap();
        assert_eq!(files.len(), 1);

        // Delete and verify it's gone
        repo.delete(&files[0]).await.unwrap();
        let files = repo
            .find_files(temp_dir.path(), FinderOptions::default())
            .await
            .unwrap();
        assert_eq!(files.len(), 0);
    }
}
