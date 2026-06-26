mod sql;
mod structure;

use crate::{
    sql::{create_sql, insert_sql},
    structure::LearningList,
};
use chrono::Local;
use rusqlite::Result;

fn main() -> Result<()> {
    let mut learning_list_table = create_sql()?;

    let add_list = LearningList {
        id: 0,
        url: String::from("example.com"),
        title: String::from("example"),
        memo: String::from("test"),
        date: Local::now(),
    };

    let _ = insert_sql(learning_list_table, add_list);
    Ok(())
}
