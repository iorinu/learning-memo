use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "learning-list", version, about = "cli learning list memo")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    //add learning list
    #[command(alias = "a")]
    Add {
        url: String,
        title: String,
        memo: String,
    },
    //view all table
    #[command(alias = "all")]
    Allview,
}
