use clap::{command, Arg, ArgMatches, Command};

pub struct Args {}

impl Args {
    pub fn new() -> ArgMatches {
        command!()
            .subcommand(Command::new("cat-file").arg(Arg::new("path").required(true).short('p')))
            .subcommand(Command::new("init"))
            .subcommand(Command::new("hash-object").arg(Arg::new("path").required(true).short('w')))
            .get_matches()
    }
}
