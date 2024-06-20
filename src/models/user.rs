use rusqlite::Row;

use crate::{query::Queryable, types::Date};

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Date,
    pub is_active: i32,
    pub created_at: i64,
}

impl Queryable for User {
    type Row = (i32, String, String, Date, i32, i64);

    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(User {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            date_of_birth: row.get(3)?,
            is_active: row.get(4)?,
            created_at: row.get(5)?,
        })
    }
}
