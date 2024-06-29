use std::io::prelude::*;
use std::{fs, io::BufReader};

use crate::compression::decode;

pub fn get_file_path(hash: String) -> String {
    format!(".git/objects/{}/{}", &hash[..2], &hash[2..])
}

pub fn get_file_buffer(file: fs::File) -> Vec<u8> {
    let mut reader = BufReader::new(file);

    let mut buffer = [0u8; 1024];
    let bytes_read = reader.read(&mut buffer).unwrap();

    decode(&buffer[..bytes_read])
}

pub fn cat_file(hash: String) -> String {
    let path = get_file_path(hash);

    let file = fs::File::open(path).unwrap();

    let buffer = get_file_buffer(file);

    let content = String::from_utf8_lossy(&buffer);
    let splitted = content.split('\0').collect::<Vec<&str>>();

    let content = *splitted.get(1).unwrap();

    content.to_owned()
}
