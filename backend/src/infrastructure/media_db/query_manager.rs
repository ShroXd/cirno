use anyhow::*;
use async_trait::async_trait;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    fs::{read_dir, read_to_string},
    sync::RwLock,
};
use tracing::*;

#[async_trait]
pub trait QueryManager: Send + Sync {
    async fn get_query(&self, entity: &str, name: &str) -> Result<String>;
    async fn reload(&self) -> Result<()>;
}

#[derive(Clone)]
pub struct FileQueryManager {
    queries: Arc<RwLock<HashMap<String, String>>>,
    sql_root_path: PathBuf,
}

impl FileQueryManager {
    pub async fn new<P: AsRef<Path>>(sql_root_path: P) -> Result<Self> {
        let manager = Self {
            queries: Arc::new(RwLock::new(HashMap::new())),
            sql_root_path: sql_root_path.as_ref().to_path_buf(),
        };

        Ok(manager)
    }

    #[instrument(skip(self))]
    async fn load_all_queries(&self) -> Result<()> {
        let mut queries = HashMap::new();
        let mut dirs = read_dir(&self.sql_root_path).await?;

        while let Some(entry) = dirs.next_entry().await? {
            let file_type = entry.file_type().await?;

            // Process SQL files based on directory structure:
            // - If entry is a directory, recursively load all SQL files within it
            //   using the directory name as the entity name
            // - If entry is a SQL file, load it directly using filename (minus .sql)
            //   as the entity name
            if file_type.is_dir() {
                let entity_path = entry.path();
                let entity_name = entry.file_name();
                self.load_entity_queries(
                    &entity_path,
                    &entity_name.to_string_lossy(),
                    &mut queries,
                )
                .await?;
            } else if file_type.is_file() {
                let file_name = entry.file_name();
                let entity_name = file_name
                    .to_string_lossy()
                    .trim_end_matches(".sql")
                    .to_string();
                if file_name.to_string_lossy().ends_with(".sql") {
                    let content = read_to_string(entry.path()).await?;
                    self.parse_named_query(&entity_name, &content, &mut queries)
                        .await?;
                }
            }
        }

        // Replace the cache with the new queries
        let mut cache = self.queries.write().await;
        *cache = queries;

        Ok(())
    }

    #[instrument(skip(self, queries))]
    async fn load_entity_queries(
        &self,
        entity_path: &PathBuf,
        entity_name: &str,
        queries: &mut HashMap<String, String>,
    ) -> Result<()> {
        let mut files = read_dir(entity_path).await?;

        while let Some(file) = files.next_entry().await? {
            if !file.file_type().await?.is_file() {
                continue;
            }

            debug!("Loading query file: {:?}", file.path());

            let file_name = file.file_name();
            if !file_name.to_string_lossy().ends_with(".sql") {
                continue;
            }

            let content = read_to_string(file.path()).await?;
            self.parse_named_query(entity_name, &content, queries)
                .await?;
        }

        Ok(())
    }

    #[instrument(skip(self, queries))]
    async fn parse_named_query(
        &self,
        entity_name: &str,
        content: &str,
        queries: &mut HashMap<String, String>,
    ) -> Result<()> {
        let mut current_query = String::new();
        let mut current_name = None;

        for line in content.lines() {
            if line.trim().starts_with("-- name:") {
                if let Some(name) = current_name.take() {
                    let key = self.build_key(entity_name, name);
                    queries.insert(key, current_query.trim().to_string());
                    current_query.clear();
                }

                current_name = Some(line.trim().strip_prefix("-- name:").unwrap().trim());
            } else if !line.trim().starts_with("--") {
                current_query.push_str(line);
                current_query.push('\n');
            }
        }

        if let Some(name) = current_name.take() {
            let key = self.build_key(entity_name, name);
            queries.insert(key, current_query.trim().to_string());
        }

        Ok(())
    }

    #[instrument(skip(self))]
    fn build_key(&self, entity: &str, name: &str) -> String {
        format!("{}.{}", entity, name)
    }
}

#[async_trait]
impl QueryManager for FileQueryManager {
    #[instrument(skip(self))]
    async fn get_query(&self, entity: &str, name: &str) -> Result<String> {
        let cache = self.queries.read().await;
        let key = self.build_key(entity, name);

        debug!("Getting query: {:?}", key);
        cache
            .get(&key)
            .cloned()
            .ok_or(anyhow!("Query not found: {}", key))
    }

    #[instrument(skip(self))]
    async fn reload(&self) -> Result<()> {
        self.load_all_queries().await
    }
}
