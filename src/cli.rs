use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lmemo", version, about = "cli learning memo")]
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
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long, default_value(""))]
        memo: String,
    },
    //view all table
    #[command(alias = "all")]
    #[command(alias = "la")]
    Allview {
        #[arg(short, long, default_value(""))]
        site: String,
    },

    #[command(alias = "v")]
    #[command(alias = "ls")]
    View,

    #[command(alias = "c")]
    Chart,

    #[command(alias = "sv")]
    SiteView,
}
