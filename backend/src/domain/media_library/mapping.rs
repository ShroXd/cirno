use sqlx::{sqlite::SqliteRow, Row};

use crate::{
    interfaces::{dtos::MediaLibraryDto, http_api::controllers::api_models::MediaLibraryCategory},
    shared::util_traits::SqliteRowMapper,
};

// TODO: consider if we can use try_get for non-panicking mapping
// Maybe we can filter out the invalid ones at domain layer and return appropriate http code to frontend
// This can make sure the program doesn't panic and also we can handle the error in a better way
impl SqliteRowMapper<MediaLibraryDto> for MediaLibraryDto {
    fn map_row(row: SqliteRow) -> Self {
        MediaLibraryDto {
            id: row.get::<i64, _>("id"),
            name: row.get::<String, _>("name"),
            category: MediaLibraryCategory::try_from(row.get::<i64, _>("category_id")).unwrap(),
        }
    }
}
