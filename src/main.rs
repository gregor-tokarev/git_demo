use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;

use args::Args;
use commands::cat_file;
use commands::init::init_command;
use flate2::bufread::ZlibDecoder;

mod args;
mod commands;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let args = Args::new();

    // Uncomment this block to pass the first stage
    if let Some(_) = args.subcommand_matches("init") {
        init_command();
    };

    if let Some(cmd) = args.subcommand_matches("cat-file") {
        let hash = cmd.get_one::<String>("path").unwrap();
        cat_file(hash);
    }
}
