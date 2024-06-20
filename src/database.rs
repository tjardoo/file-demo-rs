use rusqlite::Connection;

use crate::{models::user::User, query_map_macro};

pub fn users() -> Vec<User> {
    let conn = Connection::open("database.db").unwrap();

    create_table_if_not_exists(&conn);

    insert_into_table_if_empty(&conn);

    let users = get_users_from_table(&conn);

    users
}

fn create_table_if_not_exists(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            date_of_birth TEXT NOT NULL,
            is_active INTEGER NOT NULL,
            created_at INTEGER NOT NULL
        )",
        [],
    )
    .unwrap();
}

fn insert_into_table_if_empty(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users").unwrap();

    let count: i32 = stmt.query_row([], |row| row.get(0)).unwrap();

    if count > 0 {
        return;
    }

    conn.execute(
        "INSERT INTO users
        (first_name, last_name, date_of_birth, is_active, created_at)
        VALUES
        ('Alice', 'Smith', '1985-06-15', 1, 1622505600),
        ('Bob', 'Johnson', '1992-09-23', 1, 1625097600),
        ('Carol', 'Williams', '1988-12-04', 0, 1627689600),
        ('David', 'Brown', '1975-02-17', 0, 1630281600),
        ('Eve', 'Jones', '1995-07-21', 1, 1632873600),
        ('Frank', 'Garcia', '1980-11-29', 0, 1635465600),
        ('Grace', 'Miller', '1999-03-05', 1, 1638057600),
        ('Hank', 'Davis', '1983-08-12', 1, 1640649600),
        ('Ivy', 'Martinez', '1991-05-18', 0, 1643241600),
        ('Jack', 'Rodriguez', '1987-10-30', 0, 1645833600)
        ",
        [],
    )
    .unwrap();
}

fn get_users_from_table(conn: &Connection) -> Vec<User> {
    let mut stmt = conn
        .prepare(
            "SELECT id, first_name, last_name, date_of_birth, is_active, created_at FROM users",
        )
        .unwrap();

    let mapped_rows = query_map_macro!(stmt, User);

    mapped_rows
        .collect::<Result<Vec<User>, rusqlite::Error>>()
        .unwrap()
}
