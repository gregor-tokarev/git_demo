use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

pub fn get_hash(content: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content.as_bytes());

    let hash = hasher.finalize();

    hex::encode(hash)
}

fn compress(content: String) -> Vec<u8> {
    let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
    compressor.write_all(content.as_bytes()).unwrap();

    compressor.finish().unwrap()
}

pub fn hash_object(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content = format!("blob {}\0{}", content.len(), content);

    let hash = get_hash(content.clone());

    let dir = &hash[..2];
    let file_name = &hash[2..];

    let s = format!(".git/objects/{}", dir);
    let path = Path::new(&s);
    if !path.exists() {
        fs::create_dir(path).unwrap();
    };

    let mut blob_obj_file = fs::File::options()
        .create(true)
        .write(true)
        .open(format!("{}/{}", s, file_name))
        .unwrap();

    let compressed = compress(content);
    blob_obj_file.write_all(&compressed).unwrap();

    hash
}
