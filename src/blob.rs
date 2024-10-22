use std::{fmt::Display, str::from_utf8};

use crate::{bytes_reader::BytesReader, codec, hash::Hash, object::Header};

#[derive(Debug)]
pub struct Blob {
    pub content: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }

    pub fn parse(reader: &mut BytesReader) -> Self {
        let content = reader.read_all();
        Self::new(content.to_vec())
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Header::new("blob", self.content.len()).encode();
        bytes.extend_from_slice(&self.content);
        bytes
    }

    pub fn encode(&self) -> (Hash, Vec<u8>) {
        let bytes = self.serialize();
        let hash = Hash::hash(&bytes);
        let encoded = codec::compress(&bytes);
        (hash, encoded)
    }
}

impl Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.content).unwrap())
    }
}
