use args::Args;

use commands::cat_file::cat_file;
use commands::hash_object::hash_object;
use commands::init::init_command;

mod args;
mod commands;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let args = Args::new();

    // Uncomment this block to pass the first stage
    if args.subcommand_matches("init").is_some() {
        init_command();
        println!("Initialized git directory");
    };

    if let Some(cmd) = args.subcommand_matches("cat-file") {
        let hash = cmd.get_one::<String>("path").unwrap();
        let content = cat_file(hash.to_owned());

        print!("{content}");
    };

    if let Some(cmd) = args.subcommand_matches("hash-object") {
        let path = cmd.get_one::<String>("path").unwrap();

        hash_object(path);
    };
}
