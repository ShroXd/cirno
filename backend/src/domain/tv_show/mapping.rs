use sqlx::{sqlite::SqliteRow, Row};

use crate::{interfaces::dtos::SeasonDto, shared::util_traits::SqliteRowMapper};

impl SqliteRowMapper<SeasonDto> for SeasonDto {
    fn map_row(row: SqliteRow) -> Self {
        SeasonDto {
            season_number: Some(row.get::<i64, _>("season_number")),
            season_title: Some(row.get::<String, _>("title")),
            episodes: vec![],
        }
    }
}
