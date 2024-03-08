use rusqlite;
use rusqlite::Error;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32
}

pub fn create_db() -> Result<rusqlite::Connection,Error>{
    let db_file = "sqlite.db";
    let connect = rusqlite::Connection::open(db_file)?;
    connect.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(connect)
}



