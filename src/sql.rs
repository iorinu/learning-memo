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
            date TEXT
        )",
        (), // empty list of parameters.
    )?;
    Ok(conn)
}

pub fn insert_sql(list: &Connection, add_list: LearningList) -> Result<()> {
    list.execute(
        "INSERT INTO learning_list (url, title, memo, date) VALUES (?1, ?2, ?3, ?4)",
        (add_list.url, add_list.title, add_list.memo, add_list.date),
    )?;
    Ok(())
}

pub fn view_all_table(list: &Connection) -> Result<Vec<LearningList>> {
    let mut stmt = list.prepare("SELECT id, url, title, memo, date FROM learning_list")?;
    let rows = stmt.query_map([], |row| {
        Ok(LearningList {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            memo: row.get(3)?,
            date: row.get(4)?,
        })
    })?;

    let mut view_list = Vec::new();
    for item in rows {
        view_list.push(item?);
    }
    Ok(view_list)
}
