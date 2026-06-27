mod cli;
mod sql;
mod structure;

use std::ops::Add;

use crate::{
    cli::Cli,
    sql::{create_sql, insert_sql, select_all_table, view_table},
    structure::LearningList,
};
use chrono::Local;
use clap::Parser;
use rusqlite::Result;

fn main() -> Result<()> {
    let mut learning_list_table = create_sql()?;

    let cli = Cli::parse();

    match cli.command {
        cli::Command::Add { url, title, memo } => {
            let add_list = LearningList {
                //idは無視される
                id: 0,
                url,
                title,
                memo,
                date: Local::now(),
            };

            let _ = insert_sql(&learning_list_table, add_list);
        }
        cli::Command::Allview => {
            let all_tavle = select_all_table(&learning_list_table)?;
            let _ = view_table(&all_tavle);
        }
    }

    Ok(())
}
