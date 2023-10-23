use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let connection = Connection::open("database.sqlite3")?;

    // Create a table
    connection.execute("CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER)", [])?;

    // Insert a new user
    connection.execute("INSERT INTO users VALUES ('Alice', 42)", [])?;

    // Read a user
    let mut user = connection.query_row("SELECT * FROM users WHERE name = ?", ["Alice"])?;
    let name = user.get::<_, String>(0)?;
    let age = user.get::<_, i32>(1)?;

    // Update a user
    connection.execute("UPDATE users SET age = 43 WHERE name = ?", ["Alice"])?;

    // Delete a user
    connection.execute("DELETE FROM users WHERE name = ?", ["Alice"])?;

    Ok(())
}

    user_iter.collect("Alice")

