use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(name = "lmemo", version, about = "cli learning memo")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(alias = "a")]
    Add {
        url: String,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long, default_value(""))]
        memo: String,
    },

    #[command(alias = "all")]
    #[command(alias = "la")]
    Allview {
        #[arg(short, long)]
        site: Option<String>,
    },

    #[command(alias = "v")]
    #[command(alias = "ls")]
    View,

    #[command(alias = "c")]
    Chart,

    #[command(alias = "sv")]
    SiteView,

    #[command(alias = "o")]
    Open { id: i64 },

    /// シェル補完スクリプトを生成
    Completion { shell: Shell },
}
