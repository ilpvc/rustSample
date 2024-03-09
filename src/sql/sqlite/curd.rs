use std::env::VarError;
use std::ptr::null;
use rusqlite;
use rusqlite::{Connection, Error};

#[derive(Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

pub fn create_connect() -> Connection {
    let db_file = "sqlite.db";
    let connection = Connection::open(db_file);
    connection.unwrap()
}

pub fn create_table() -> Result<Connection, Error> {
    let connect = create_connect();
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


pub fn insert_person(person: Person) {
    let connect = create_connect();
    connect.execute(
        "INSERT INTO person (name,age) values (?1,?2)",
        [person.name, person.age.to_string()],
    ).unwrap();
}

pub fn select_all_person() -> Result<Vec<Person>, Error> {
    let connect = create_connect();
    let mut stmt = connect.prepare(
        "select * from person"
    )?;
    let person_list = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            age: row.get(2).unwrap(),
        })
    })?;
    let mut persons: Vec<Person> = Vec::new();
    for person in person_list {
        persons.push(person.unwrap());
    };
    Ok(persons)
}




