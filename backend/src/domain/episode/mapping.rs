use sqlx::{sqlite::SqliteRow, Row};

use crate::{interfaces::dtos::EpisodeDto, shared::util_traits::SqliteRowMapper};

impl SqliteRowMapper<EpisodeDto> for EpisodeDto {
    fn from_row(row: SqliteRow) -> Self {
        EpisodeDto {
            id: row.get::<i64, _>("id"),
            title: row.get::<Option<String>, _>("title"),
            original_title: row.get::<Option<String>, _>("original_title"),
            plot: row.get::<Option<String>, _>("plot"),
            nfo_path: row.get::<Option<String>, _>("nfo_path"),
            video_file_path: row.get::<String, _>("video_file_path"),
            subtitle_file_path: row.get::<Option<String>, _>("subtitle_file_path"),
            thumb_image_url: row.get::<Option<String>, _>("thumb_image_url"),
            thumb_image: row.get::<Option<String>, _>("thumb_image"),
            episode_number: row.get::<Option<i64>, _>("episode_number"),
            runtime: row.get::<Option<i64>, _>("runtime"),
            season_number: row.get::<Option<i64>, _>("season_number"),
            season_title: row.get::<Option<String>, _>("season_title"),
        }
    }
}
