use rusqlite::Connection;
use rusqlite::{Connection, NO_PARAMS};

fn main() {
    let conn = Connection::open("my_database.db").expect("Failed to open database");
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    )
    .expect("Failed to create table");
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

fn create_user(conn: &Connection, name: &str, email: &str) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        &[name, email],
    )
}

fn read_users(conn: &Connection) -> rusqlite::Result<Vec<User>> {
    let mut stmt = conn.prepare("SELECT id, name, email FROM users")?;
    let user_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    })?;

    user_iter.collect()
}
