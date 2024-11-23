use sqlx::sqlite::SqliteRow;

pub trait SqliteRowMapper<T> {
    fn map_row(row: SqliteRow) -> T;
}

pub fn map_rows<T>(rows: Vec<SqliteRow>) -> Vec<T>
where
    T: SqliteRowMapper<T>,
{
    rows.into_iter().map(T::map_row).collect()
}
