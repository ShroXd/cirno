use sqlx::{sqlite::SqliteRow, Row};

use crate::{domain::media_actor::model::MediaActor, shared::util_traits::SqliteRowMapper};

impl SqliteRowMapper<MediaActor> for MediaActor {
    fn from_row(row: SqliteRow) -> Self {
        MediaActor {
            name: row.get::<Option<String>, _>("name"),
            role: row.get::<Option<String>, _>("role"),
            thumb: row.get::<Option<String>, _>("thumb"),
            profile: row.get::<Option<String>, _>("profile"),
            tmdb_id: row.get::<Option<String>, _>("tmdb_id"),
        }
    }
}
