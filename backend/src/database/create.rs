use anyhow::*;
use rayon::prelude::*;
use sqlx::{Acquire, SqliteConnection, SqlitePool};
use tracing::*;

use crate::{
    handlers::media_library::CreateMediaLibraryPayload,
    services::library_parser::parsers::{Episode, Season, TVSerie},
};

// TODO: the code for checking duplicated data is still not correct
pub async fn insert_tv_series(conn_pool: &SqlitePool, tv_series: &TVSerie) -> Result<()> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    sqlx::query!(
        "INSERT OR IGNORE INTO tv_series (title, nfo_path, poster_path, fanart_path, country, year, plot, tmdb_id, imdb_id, wikidata_id, tvdb_id)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        tv_series.title,
        tv_series.nfo_path,
        tv_series.poster_path,
        tv_series.fanart_path,
        tv_series.country,
        tv_series.year,
        tv_series.plot,
        tv_series.tmdb_id,
        tv_series.imdb_id,
        tv_series.wikidata_id,
        tv_series.tvdb_id,
    )
    .execute(&mut *tx)
    .await?;

    // Get the series_id whether it was just inserted or already existed
    let series_id: i64 =
        sqlx::query_scalar!("SELECT id FROM tv_series WHERE title = ?", tv_series.title)
            .fetch_one(&mut *tx)
            .await?
            .ok_or(anyhow::anyhow!("Failed to get series ID"))?;
    debug!("Series ID: {}", series_id);

    for genre in &tv_series.genres {
        debug!("Inserting genre: {}", genre);
        sqlx::query!("INSERT OR IGNORE INTO genres (name) VALUES (?)", genre)
            .execute(&mut *tx)
            .await?;

        let genre_id: i64 = sqlx::query_scalar!("SELECT id FROM genres WHERE name = ?", genre)
            .fetch_one(&mut *tx)
            .await?
            .ok_or(anyhow::anyhow!("Failed to get genre ID"))?;

        debug!("Linking genre {} to series {}", genre_id, series_id);
        sqlx::query!(
            "INSERT OR IGNORE INTO tv_series_genres (series_id, genre_id) VALUES (?, ?)",
            series_id,
            genre_id,
        )
        .execute(&mut *tx)
        .await?;
    }

    for actor in &tv_series.actors {
        debug!("Inserting actor: {:?}", actor.name);
        sqlx::query!(
            "INSERT OR IGNORE INTO actors (name, role, thumb, profile, tmdb_id) VALUES (?, ?, ?, ?, ?)",
            actor.name,
            actor.role,
            actor.thumb,
            actor.profile,
            actor.tmdb_id,
        )
        .execute(&mut *tx)
        .await?;

        let actor_id: i64 =
            sqlx::query_scalar!("SELECT id FROM actors WHERE name = ?", actor.name,)
                .fetch_one(&mut *tx)
                .await?
                .ok_or(anyhow::anyhow!("Failed to get actor ID"))?;

        debug!("Linking actor {} to series {}", actor_id, series_id);
        sqlx::query!(
            "INSERT OR IGNORE INTO tv_series_actors (series_id, actor_id) VALUES (?, ?)",
            series_id,
            actor_id,
        )
        .execute(&mut *tx)
        .await?;
    }

    let seasons = tv_series
        .seasons
        .par_iter()
        .map(|(_, season)| season)
        .collect::<Vec<&Season>>();

    for season in seasons {
        insert_season(&mut *tx, series_id, season).await?;
    }

    tx.commit().await?;
    debug!("Committed transaction");
    Ok(())
}

pub async fn insert_season(
    tx: &mut SqliteConnection,
    series_id: i64,
    season: &Season,
) -> Result<()> {
    // Check if the season already exists
    let existing_season_id: Option<i64> = sqlx::query_scalar!(
        "SELECT id FROM seasons WHERE series_id = ? AND season_number = ?",
        series_id,
        season.season_number,
    )
    .fetch_optional(&mut *tx)
    .await?;

    if existing_season_id.is_some() {
        debug!("Season already exists");
        return Ok(());
    }

    sqlx::query!(
        "INSERT OR IGNORE INTO seasons (series_id, season_number, title, plot, nfo_path)
         VALUES (?, ?, ?, ?, ?)",
        series_id,
        season.season_number,
        season.title,
        season.plot,
        season.nfo_path,
    )
    .execute(&mut *tx)
    .await?;

    let season_id: i64 = match sqlx::query_scalar!(
        "SELECT id FROM seasons WHERE series_id = ? AND season_number = ?",
        series_id,
        season.season_number,
    )
    .fetch_optional(&mut *tx)
    .await?
    {
        Some(id) => id,
        None => {
            error!("Failed to get season ID");
            return Err(anyhow::anyhow!("Failed to get season ID"));
        }
    };
    debug!("Season ID: {}", season_id);

    let episodes = season
        .episodes
        .par_iter()
        .map(|(_, episode)| episode)
        .collect::<Vec<&Episode>>();

    for episode in episodes {
        insert_episode(
            &mut *tx,
            series_id,
            season_id,
            season.season_number.unwrap_or(0) as i64,
            episode,
        )
        .await?;
    }

    Ok(())
}

pub async fn insert_episode(
    tx: &mut SqliteConnection,
    series_id: i64,
    season_id: i64,
    season_number: i64,
    episode: &Episode,
) -> Result<()> {
    // Check if the episode already exists
    let existing_episode_id: Option<i64> = sqlx::query_scalar!(
        "SELECT id FROM episodes WHERE series_id = ? AND season_id = ? AND title = ?",
        series_id,
        season_id,
        episode.title,
    )
    .fetch_optional(&mut *tx)
    .await?;

    debug!("Existing episode ID: {}", existing_episode_id.unwrap_or(-1));

    if existing_episode_id.is_some() {
        debug!("Episode already exists");
        return Ok(());
    }

    sqlx::query!(
        "INSERT OR IGNORE INTO episodes (series_id, season_id, title, original_title, plot, nfo_path, video_file_path, subtitle_file_path, thumb_image_url, thumb_image, season_number, episodes_number, runtime)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        series_id,
        season_id,
        episode.title,
        episode.original_title,
        episode.plot,
        episode.nfo_path,
        episode.video_file_path,
        episode.subtitle_file_path,
        episode.thumb_image_url,
        episode.thumb_image,
        season_number,
        episode.episode_number,
        episode.runtime,
    )
    .execute(&mut *tx)
    .await?;

    Ok(())
}

pub async fn insert_media_library(
    conn_pool: &SqlitePool,
    media_library: &CreateMediaLibraryPayload,
) -> Result<i64> {
    let mut conn = conn_pool.acquire().await?;
    let mut tx = conn.begin().await?;

    let category_id = i32::from(media_library.category.clone());
    debug!("Category ID: {}", category_id);

    let existing_category_id: Option<i64> =
        sqlx::query_scalar!("SELECT id FROM category_mapping WHERE id = ?", category_id,)
            .fetch_optional(&mut *tx)
            .await?;
    if existing_category_id.is_none() {
        return Err(anyhow::anyhow!("Category does not exist"));
    }

    let media_library_id: i64 = sqlx::query_scalar!(
        "INSERT OR IGNORE INTO media_library (name, directory, category_id) VALUES (?, ?, ?) RETURNING id",
        media_library.name,
        media_library.directory,
        category_id,
    )
        .fetch_one(&mut *tx)
        .await?
        .ok_or(anyhow::anyhow!("Failed to get media library ID"))?;

    tx.commit().await?;
    Ok(media_library_id)
}
