use std::{fmt::Display, slice::Iter};

use crate::{bytes_reader::BytesReader, codec, hash::Hash, object::Header};

#[derive(Debug)]
pub struct TreeNodeEntry {
    pub mode: String,
    pub name: String,
    pub hash: Hash,
}

impl TreeNodeEntry {
    pub fn new(mode: String, name: String, hash: Hash) -> Self {
        Self { mode, name, hash }
    }

    fn parse(reader: &mut BytesReader) -> Self {
        let mode = reader.read_until(b' ');
        reader.skip();
        let name = reader.read_until(0);
        reader.skip();
        let hash = reader.read_n(20);

        Self::new(
            String::from_utf8(mode.to_vec()).unwrap(),
            String::from_utf8(name.to_vec()).unwrap(),
            Hash::new(hash.try_into().unwrap()),
        )
    }

    pub fn encode(&self) -> Vec<u8> {
        format!("{} {}\0", self.mode, self.name)
            .bytes()
            .chain(self.hash.bytes())
            .collect()
    }
}

impl Display for TreeNodeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>6} {}\t{}", self.mode, self.hash, self.name)
    }
}

#[derive(Debug)]
pub struct TreeNode {
    entries: Vec<TreeNodeEntry>,
}

impl TreeNode {
    pub fn new(entries: Vec<TreeNodeEntry>) -> Self {
        Self { entries }
    }

    pub fn parse(reader: &mut BytesReader) -> Self {
        let mut entries = vec![];
        while !reader.is_at_end() {
            entries.push(TreeNodeEntry::parse(reader));
        }
        Self::new(entries)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let payload: Vec<_> = self
            .entries
            .iter()
            .flat_map(|entry| entry.encode())
            .collect();
        let mut bytes = Header::new("tree", payload.len()).encode();
        bytes.extend_from_slice(&payload);
        bytes
    }

    pub fn encode(&self) -> (Hash, Vec<u8>) {
        let bytes = self.serialize();
        let hash = Hash::hash(&bytes);
        let encoded = codec::compress(&bytes);
        (hash, encoded)
    }
}

impl<'a> IntoIterator for &'a TreeNode {
    type Item = &'a TreeNodeEntry;
    type IntoIter = Iter<'a, TreeNodeEntry>;
    fn into_iter(self) -> Self::IntoIter {
        let entries = &self.entries;
        entries.into_iter()
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self {
            writeln!(f, "{}", entry)?
        }
        Ok(())
    }
}
