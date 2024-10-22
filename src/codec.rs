use std::io::Read;

use flate2::bufread::ZlibDecoder;

pub fn decompress(compressed: &[u8]) -> Vec<u8> {
    let mut bytes = vec![];
    ZlibDecoder::new(&compressed[..])
        .read_to_end(&mut bytes)
        .unwrap();
    bytes
}
