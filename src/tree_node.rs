use std::{fmt::Display, slice::Iter};

use crate::{bytes_reader::BytesReader, hash::Hash};

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
