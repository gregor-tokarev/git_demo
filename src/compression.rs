use std::io::prelude::*;

use flate2::{bufread::ZlibDecoder, write::ZlibEncoder, Compression};

pub fn encode(content: String) -> Vec<u8> {
    let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
    compressor.write_all(content.as_bytes()).unwrap();

    compressor.finish().unwrap()
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(bytes);
    let mut buffer = [0u8; 1024];
    let bytes_read = decoder.read(&mut buffer).unwrap();

    buffer[..bytes_read].to_vec()
}
