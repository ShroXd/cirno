use sqlx::sqlite::SqliteRow;

pub trait SqliteRowMapper<T> {
    fn from_row(row: SqliteRow) -> T;
}

pub fn map_rows<T>(rows: Vec<SqliteRow>) -> Vec<T>
where
    T: SqliteRowMapper<T>,
{
    rows.into_iter().map(T::from_row).collect()
}
