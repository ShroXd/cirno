use sqlx::{sqlite::SqliteRow, Row};

use crate::{
    interfaces::http_api::controllers::api_models::MediaLibraryCategory,
    shared::util_traits::SqliteRowMapper,
};

use super::model::{MediaLibraryBrief, MediaLibraryPoster};

// TODO: consider if we can use try_get for non-panicking mapping
// Maybe we can filter out the invalid ones at domain layer and return appropriate http code to frontend
// This can make sure the program doesn't panic and also we can handle the error in a better way
impl SqliteRowMapper<MediaLibraryBrief> for MediaLibraryBrief {
    fn from_row(row: SqliteRow) -> Self {
        MediaLibraryBrief {
            id: row.get::<i64, _>("id"),
            name: row.get::<String, _>("name"),
            category: MediaLibraryCategory::try_from(row.get::<i64, _>("category_id")).unwrap(),
        }
    }
}

impl SqliteRowMapper<MediaLibraryPoster> for MediaLibraryPoster {
    fn from_row(row: SqliteRow) -> Self {
        MediaLibraryPoster {
            id: row.get::<i64, _>("id"),
            poster_path: row.get::<Option<String>, _>("poster_path"),
        }
    }
}
