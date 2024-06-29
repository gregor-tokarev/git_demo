use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    CatFile {
        #[clap(short = 'p')]
        object: String,
    },
    HashObject {
        #[clap(short = 'w')]
        path: PathBuf,
    },
    LsTree {
        #[clap(long = "name-only")]
        name_only: bool,

        object: String,
    },
    WriteTree
}
