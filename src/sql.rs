use crate::structure::LearningList;

use rusqlite::{Connection, Result};

pub fn create_sql() -> Result<Connection> {
    let conn = Connection::open("learning_list.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS learning_list (
            id    INTEGER PRIMARY KEY,
            url TEXT,
            title TEXT,
            memo TEXT,
            date TEXT,
        )",
        (), // empty list of parameters.
    )?;
    Ok(conn)
}

pub fn insert_sql(list: Connection, add_list: LearningList) -> Result<()> {
    list.execute(
        "INSERT INTO learning_list (url, title, memo, date) VALUES (?1, ?2, ?3, ?4)",
        (add_list.url, add_list.title, add_list.memo, add_list.date),
    )?;
    Ok(())
}
