use std::path::PathBuf;

use clap::{command, Arg, ArgMatches, Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    CatFile {
        #[clap(short = 'p')]
        path: String
    },
    HashObject {
        #[clap(short = 'w')]
        file: PathBuf
    },
}

// impl Args {
//     pub fn new() -> ArgMatches {
//         command!()
//             .subcommand(Command::new("cat-file").arg(Arg::new("path").required(true).short('p')))
//             .subcommand(Command::new("init"))
//             .subcommand(Command::new("hash-object").arg(Arg::new("path").required(true).short('w')))
//             .get_matches()
//     }
// }
