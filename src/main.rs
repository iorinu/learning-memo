mod cli;
mod fetch;
mod sql;
mod structure;

use crate::{
    cli::Cli,
    fetch::fetch_title,
    sql::{create_sql, daily_chart, insert_sql, recent_table, select_all_table, view_table},
    structure::LearningList,
};
use chrono::Local;
use clap::Parser;
use rusqlite::Result;

fn main() -> Result<()> {
    let learning_list_table = create_sql()?;

    let cli = Cli::parse();

    match cli.command {
        cli::Command::Add { url, title, memo } => {
            let title = match title {
                Some(t) => t,
                None => fetch_title(&url).unwrap_or_else(|_| "取得失敗".to_string()),
            };

            let add_list = LearningList {
                //idは無視される
                id: 0,
                url,
                title,
                memo,
                date: Local::now().date_naive(),
            };

            let _ = insert_sql(&learning_list_table, add_list);
        }
        cli::Command::Allview => {
            let all_table = select_all_table(&learning_list_table)?;
            let _ = view_table(&all_table);
        }
        cli::Command::View => {
            let table = recent_table(&learning_list_table)?;
            let _ = view_table(&table);
        }
        cli::Command::Chart => {
            let list = select_all_table(&learning_list_table)?;
            let _ = daily_chart(&list);
        }
    }

    Ok(())
}
