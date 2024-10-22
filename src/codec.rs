use std::io::Read;

use flate2::{
    bufread::{ZlibDecoder, ZlibEncoder},
    Compression,
};

pub fn decompress(compressed: &[u8]) -> Vec<u8> {
    let mut bytes = vec![];
    ZlibDecoder::new(&compressed[..])
        .read_to_end(&mut bytes)
        .unwrap();
    bytes
}

pub fn compress(bytes: &[u8]) -> Vec<u8> {
    let mut z = ZlibEncoder::new(&bytes[..], Compression::fast());
    let mut encoded = vec![];
    z.read_to_end(&mut encoded).unwrap();
    encoded
}
