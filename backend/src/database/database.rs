use anyhow::*;
use sqlx::SqlitePool;
use std::result::Result::Ok;
use tracing::*;

pub struct Database {
    file_path: &'static str,
    pool: SqlitePool,
}

impl Database {
    #[instrument]
    pub async fn new(file_path: &'static str) -> Result<Self> {
        // TODO: https://tauritutorials.com/blog/building-a-todo-app-in-tauri-with-sqlite-and-sqlx
        let pool = SqlitePool::connect(&format!("sqlite:{}", file_path)).await?;
        debug!("Initialized database at {}", file_path);

        Ok(Self { file_path, pool })
    }

    // TODO: will be deprecated
    #[instrument(skip(self))]
    pub async fn initialize_db(&self) -> Result<()> {
        // TODO: who want to write sql in string?
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS tv_series (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                title TEXT,
                description TEXT,
                nfo_path TEXT,
                poster_path TEXT,
                fanart_path TEXT,
                country TEXT,
                year INTEGER,
                plot TEXT,
                tmdb_id TEXT,
                imdb_id TEXT,
                wikidata_id TEXT,
                tvdb_id TEXT
            );
            CREATE TABLE IF NOT EXISTS seasons (
                id INTEGER PRIMARY KEY,
                series_id INTEGER NOT NULL,
                number INTEGER,
                title TEXT,
                description TEXT,
                nfo_path TEXT,
                FOREIGN KEY(series_id) REFERENCES tv_series(id)
            );
            CREATE TABLE IF NOT EXISTS episodes (
                id INTEGER PRIMARY KEY,
                season_id INTEGER NOT NULL,
                series_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                title TEXT,
                description TEXT,
                nfo_path TEXT,
                video_file TEXT NOT NULL,
                subtitle_file TEXT,
                thumbnail_image TEXT,
                FOREIGN KEY(season_id) REFERENCES seasons(id),
                FOREIGN KEY(series_id) REFERENCES tv_series(id)
            );
            CREATE TABLE IF NOT EXISTS genres (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            );
            CREATE TABLE IF NOT EXISTS tv_series_genres (
                series_id INTEGER NOT NULL,
                genre_id INTEGER NOT NULL,
                PRIMARY KEY (series_id, genre_id),
                FOREIGN KEY(series_id) REFERENCES tv_series(id),
                FOREIGN KEY(genre_id) REFERENCES genres(id)
            );
            CREATE TABLE IF NOT EXISTS tv_series_actors (
                series_id INTEGER NOT NULL,
                actor_id INTEGER NOT NULL,
                PRIMARY KEY (series_id, actor_id),
                FOREIGN KEY(series_id) REFERENCES tv_series(id),
                FOREIGN KEY(actor_id) REFERENCES actors(id)
            );
            CREATE TABLE IF NOT EXISTS actors (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                role TEXT,
                thumb TEXT,
                profile TEXT,
                tmdb_id TEXT
            );
        ",
        )
        .execute(&self.pool)
        .await?;

        debug!("Initialized database schema");

        Ok(())
    }

    #[instrument(skip(self))]
    pub fn get_connection_pool(&self) -> SqlitePool {
        self.pool.clone()
    }
}
