mod sql;
mod structure;

use crate::{
    sql::{create_sql, insert_sql, view_all_table},
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

    let _ = insert_sql(&learning_list_table, add_list);

    let all_list = view_all_table(&learning_list_table)?;

    println!("{}", all_list[0].id);
    println!("{}", all_list[0].url);
    println!("{}", all_list[0].title);
    println!("{}", all_list[0].memo);
    println!("{}", all_list[0].date);

    Ok(())
}
