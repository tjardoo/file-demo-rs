pub trait Queryable {
    type Row;

    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self>
    where
        Self: Sized;
}

#[macro_export]
macro_rules! query_map_macro {
    ($stmt:expr, $type:ty) => {{
        $stmt
            .query_map([], |row| {
                Ok(<$type as crate::query::Queryable>::from_row(row)?)
            })
            .unwrap()
    }};
}
