use anyhow::*;
use std::sync::Arc;

use crate::{
    domain::file_system::file_repository::FileRepository,
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

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;
    use mockall::predicate::eq;
    use std::path::Path;
    use std::sync::Arc;

    use super::*;
    use crate::domain::file_system::file::File;

    mock! {
        pub FileRepository {}

        #[async_trait]
        impl FileRepository for FileRepository {
            async fn find_files(&self, root_dir: &Path, options: FinderOptions) -> Result<Vec<File>>;
            async fn delete(&self, file: &File) -> Result<()>;
        }
    }

    #[tokio::test]
    async fn test_delete_single_files_in_folder_success() {
        let mut mock_file_repository = MockFileRepository::new();

        let test_file = File::new(Path::new("test_file").to_path_buf());
        let test_file_clone = test_file.clone();
        mock_file_repository
            .expect_find_files()
            .with(eq(Path::new("test_folder")), mockall::predicate::always())
            .times(1)
            .returning(move |_, _| Ok(vec![test_file_clone.clone()]));

        let test_file_for_delete = test_file.clone();
        mock_file_repository
            .expect_delete()
            .withf(move |file| file == &test_file_for_delete)
            .times(1)
            .returning(move |_| Ok(()));

        let file_service = FileService::new(Arc::new(mock_file_repository));

        let result = file_service
            .delete_files_in_folder("test_folder", FinderOptions::default())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_multiple_files_in_folder_success() {
        let mut mock_file_repository = MockFileRepository::new();

        let test_file1 = File::new(Path::new("test_file1").to_path_buf());
        let test_file2 = File::new(Path::new("test_file2").to_path_buf());

        let test_file_returned1 = test_file1.clone();
        let test_file_returned2 = test_file2.clone();

        mock_file_repository
            .expect_find_files()
            .with(eq(Path::new("test_folder")), mockall::predicate::always())
            .times(1)
            .returning(move |_, _| {
                Ok(vec![
                    test_file_returned1.clone(),
                    test_file_returned2.clone(),
                ])
            });

        let test_file_for_delete1 = test_file1.clone();
        let test_file_for_delete2 = test_file2.clone();
        mock_file_repository
            .expect_delete()
            .withf(move |file| file == &test_file_for_delete1 || file == &test_file_for_delete2)
            .times(2)
            .returning(move |_| Ok(()));

        let file_service = FileService::new(Arc::new(mock_file_repository));

        let result = file_service
            .delete_files_in_folder("test_folder", FinderOptions::default())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_files_in_folder_delete_error() {
        let mut mock_file_repository = MockFileRepository::new();

        mock_file_repository
            .expect_find_files()
            .with(eq(Path::new("test_folder")), mockall::predicate::always())
            .times(1)
            .returning(move |_, _| Err(anyhow!("Failed to find files")));

        let file_service = FileService::new(Arc::new(mock_file_repository));

        let result = file_service
            .delete_files_in_folder("test_folder", FinderOptions::default())
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Failed to find files");
    }

    #[tokio::test]
    async fn test_delete_files_in_folder_empty() {
        let mut mock_file_repository = MockFileRepository::new();

        mock_file_repository
            .expect_find_files()
            .with(eq(Path::new("test_folder")), mockall::predicate::always())
            .times(1)
            .returning(move |_, _| Ok(vec![]));

        let file_service = FileService::new(Arc::new(mock_file_repository));

        let result = file_service
            .delete_files_in_folder("test_folder", FinderOptions::default())
            .await;

        assert!(result.is_ok());
    }
}
