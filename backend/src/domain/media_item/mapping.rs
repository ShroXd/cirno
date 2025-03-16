use sqlx::{sqlite::SqliteRow, Row};

use crate::{interfaces::dtos::MediaItemDto, shared::util_traits::SqliteRowMapper};

impl SqliteRowMapper<MediaItemDto> for MediaItemDto {
    fn from_row(row: SqliteRow) -> Self {
        MediaItemDto {
            id: row.get::<i64, _>("id"),
            title: row.get::<String, _>("title"),
            original_title: row.get::<Option<String>, _>("original_title"),
            plot: row.get::<Option<String>, _>("plot"),
            poster_path: row.get::<Option<String>, _>("poster_path"),
            fanart_path: row.get::<Option<String>, _>("fanart_path"),
            country: row.get::<Option<String>, _>("country"),
            premiered: row.get::<Option<String>, _>("premiered"),
            rating: row.get::<Option<f64>, _>("rating"),
            runtime: row.get::<Option<i64>, _>("runtime"),
            year: row.get::<Option<i64>, _>("year").map(|y| y.to_string()),
            genres: row
                .get::<String, _>("genres")
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            studios: row
                .get::<String, _>("studios")
                .split(',')
                .map(|s| s.to_string())
                .collect(),
            actors: vec![],
        }
    }
}
