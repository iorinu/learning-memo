mod browser;
mod cli;
mod fetch;
mod sql;
mod structure;

use crate::{
    browser::open_url,
    cli::Cli,
    fetch::{fetch_domain, fetch_title},
    sql::{
        create_sql, daily_chart, insert_sql, recent_table, select_all_table, select_domain_table,
        view_all_site, view_table,
    },
    structure::LearningList,
};
use chrono::Local;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use rusqlite::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let learning_list_table = create_sql()?;

    let cli = Cli::parse();

    match cli.command {
        cli::Command::Add { url, title, memo } => {
            let title = match title {
                Some(t) => t,
                None => fetch_title(&url).unwrap_or_else(|_| "取得失敗".to_string()),
            };

            let domain = fetch_domain(&url);

            let add_list = LearningList {
                //idは無視される
                id: 0,
                url,
                title,
                memo,
                date: Local::now().date_naive(),
                domain,
            };

            insert_sql(&learning_list_table, add_list)?;
        }
        cli::Command::Allview { site } => {
            let list = match site {
                Some(s) => select_domain_table(&learning_list_table, &s)?,
                None => select_all_table(&learning_list_table)?,
            };
            view_table(&list)?;
        }
        cli::Command::View => {
            let table = recent_table(&learning_list_table)?;
            view_table(&table)?;
        }
        cli::Command::Chart => {
            let list = select_all_table(&learning_list_table)?;
            daily_chart(&list)?;
        }
        cli::Command::SiteView => {
            view_all_site(&learning_list_table)?;
        }

        cli::Command::Open { id } => {
            open_url(id, &learning_list_table)?;
        }

        cli::Command::Completion { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut std::io::stdout());
        }
    }

    Ok(())
}
