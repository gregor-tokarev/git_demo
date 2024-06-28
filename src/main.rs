use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::io::prelude::*;

use args::Args;
use flate2::bufread::ZlibDecoder;

mod args;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let args = Args::new();

    // Uncomment this block to pass the first stage
    if let Some(_) = args.subcommand_matches("init") {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    };

    if let Some(cat_file) = args.subcommand_matches("cat-file") {
        let hash = cat_file.get_one::<String>("path").unwrap();

        let path = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);
        let file = fs::File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        let mut buffer = [0u8; 1024];

        let bytes_read = reader.read(&mut buffer).unwrap();

        let mut decoder = ZlibDecoder::new(&buffer[..bytes_read]);
        let mut buffer = [0u8; 1024];
        let bytes_read = decoder.read(&mut buffer).unwrap();

        let content = String::from_utf8_lossy(&buffer[..bytes_read]);
        let splitted = content.split("\0").collect::<Vec<&str>>();

        let content = *splitted.get(1).unwrap();

        print!("{}", content);
    }
}
