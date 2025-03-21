use sqlx::{sqlite::SqliteRow, Row};

use crate::{
    domain::library::model::LibraryStatus,
    interfaces::http_api::controllers::api_models::LibraryCategory,
    shared::util_traits::SqliteRowMapper,
};

use super::model::{LibraryBrief, LibraryPoster};

// TODO: consider if we can use try_get for non-panicking mapping
// Maybe we can filter out the invalid ones at domain layer and return appropriate http code to frontend
// This can make sure the program doesn't panic and also we can handle the error in a better way
impl SqliteRowMapper<LibraryBrief> for LibraryBrief {
    fn from_row(row: SqliteRow) -> Self {
        LibraryBrief {
            id: row.get::<i64, _>("id"),
            name: row.get::<String, _>("name"),
            category: LibraryCategory::try_from(row.get::<i64, _>("category_id")).unwrap(),
            directory: row.get::<String, _>("directory"),
            item_count: row.get::<i64, _>("item_count"),
            last_scanned: row.get::<Option<String>, _>("last_scanned"),
            current_status: LibraryStatus::from_id(row.get::<i64, _>("current_status")),
            auto_scan: row.get::<bool, _>("auto_scan"),
            error: row.get::<Option<String>, _>("error"),
            storage_used: row.get::<i64, _>("storage_used"),
            health_score: row.get::<i64, _>("health_score"),
            created_at: row.get::<String, _>("created_at"),
            updated_at: row.get::<String, _>("updated_at"),
        }
    }
}

impl SqliteRowMapper<LibraryPoster> for LibraryPoster {
    fn from_row(row: SqliteRow) -> Self {
        LibraryPoster {
            id: row.get::<i64, _>("id"),
            poster_path: row.get::<Option<String>, _>("poster_path"),
        }
    }
}
