use std::{fmt::Display, str::from_utf8};

use crate::bytes_reader::BytesReader;

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
}

impl Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.content).unwrap())
    }
}
