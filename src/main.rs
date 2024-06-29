use std::io::prelude::*;
use std::{fs, vec};

use anyhow::Context;
use args::{Args, Command};

use clap::Parser;
use commands::cat_file::cat_file;
use commands::hash_object::hash_object;
use commands::init::init_command;
use compression::decode;
use objects::tree::{FileTree, ObjectParseError};
use objects::GitObject;

mod args;
mod commands;
mod compression;
mod objects;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let args = Args::parse();

    match args.command {
        Command::Init => {
            init_command();
            println!("Initialized git directory");
        }
        Command::CatFile { object } => {
            let content = cat_file(object.to_owned());

            print!("{}", content);
        }
        Command::HashObject { path } => {
            let hash = hash_object(path.to_str().unwrap());
            print!("{}", hash)
        }
        Command::LsTree { object, name_only } => {
            let path = format!(".git/objects/{}/{}", &object[..2], &object[2..]);
            let mut file = fs::File::open(path).unwrap();

            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();

            let content = decode(&buffer[..]);

            match FileTree::from_bytes(&content) {
                Ok(tree) => {
                    for row in tree.rows {
                        if name_only {
                            println!("{}", row.object_name);
                        } else {
                            println!("{}", row);
                        }
                    }
                }
                Err(error) => {
                    match error
                        .downcast::<ObjectParseError>()
                        .context("Not convertable into ObjectParseError")
                        .unwrap()
                    {
                        ObjectParseError::NotTree => {
                            println!("Provided object is not an tree");
                        }
                    }
                }
            };
        },
        Command::WriteTree => {
            let dir = fs::read_dir("./").unwrap();
            for f in dir {
                println!("{:?}", f.unwrap());
            }
        }
    }
}
