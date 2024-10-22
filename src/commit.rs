use chrono::Local;

use crate::{bytes_reader::BytesReader, codec, hash::Hash, object::Header};

#[derive(Debug)]
pub struct Commit {
    tree: Hash,
    parents: Vec<Hash>,
    name: String,
    email: String,
    timestamp: String,
    timezone: String,
    message: String,
}

impl Commit {
    pub fn new(
        tree: Hash,
        parents: Vec<Hash>,
        timestamp: String,
        timezone: String,
        message: String,
    ) -> Self {
        Self {
            tree,
            parents,
            name: String::from("Name"),
            email: String::from("name@example.com"),
            timestamp,
            timezone,
            message,
        }
    }

    pub fn new_current_time(tree: Hash, parents: Vec<Hash>, message: String) -> Self {
        let now = Local::now();
        let timestamp = now.timestamp().to_string();
        let offset = now.offset().to_string().replace(":", "");
        Self::new(tree, parents, timestamp, offset, message)
    }

    pub fn parse(_reader: &mut BytesReader) -> Self {
        unimplemented!()
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut payload = vec![];

        payload.push(format!("tree {}", self.tree));
        for parent in &self.parents {
            payload.push(format!("parent {}", parent));
        }
        let user = format!("{} <{}>", self.name, self.email);
        let dt = format!("{} {}", self.timestamp, self.timezone);
        payload.push(format!("author {} {}", user, dt));
        payload.push(format!("committer {} {}", user, dt));

        let payload = format!("{}\n\n{}\n", payload.join("\n"), self.message);
        let payload: Vec<_> = payload.bytes().collect();

        let mut bytes = Header::new("commit", payload.len()).encode();
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

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{object::Object, reference_impl, repo::Repo, test_utils, tree::write_tree};

    use super::Commit;

    #[test]
    fn test_create_commit() {
        let root = test_utils::create_test_dir();
        let repo = Repo::new(&root);
        repo.init();

        let filename = "hello.txt";
        let contents = "Hello World!";
        fs::write(root.join(filename), contents).unwrap();

        let tree_want = write_tree(&root).unwrap();
        let parents_want = vec![];
        let message_want = String::from("msg");
        let commit = Commit::new_current_time(tree_want, parents_want, message_want.clone());
        let hash = Object::Commit(commit).write(&root);

        let (tree_got, parents_got, message_got) =
            reference_impl::read_commit(root, &hash.to_string());
        assert_eq!(tree_got, tree_want.to_string());
        assert_eq!(parents_got.len(), 0);
        assert_eq!(message_got.unwrap(), format!("{}\n", message_want));
    }
}
