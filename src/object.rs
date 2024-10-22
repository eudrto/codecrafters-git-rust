use std::{path::Path, str::from_utf8};

use crate::{
    blob::Blob, bytes_reader::BytesReader, codec, hash::Hash, input_output, tree_node::TreeNode,
};

pub struct Header<'a> {
    pub kind: &'a str,
    pub size: usize,
}

impl<'a> Header<'a> {
    pub fn new(kind: &'a str, size: usize) -> Self {
        Self { kind, size }
    }

    pub fn parse(reader: &mut BytesReader<'a>) -> Self {
        let kind = reader.read_until(b' ');
        let kind = from_utf8(kind).unwrap();
        reader.skip();
        let size = reader.read_until(0);
        let size = from_utf8(size).unwrap().parse::<usize>().unwrap();
        reader.skip();
        Self::new(kind, size)
    }

    pub fn encode(&self) -> Vec<u8> {
        format!("{} {}\0", self.kind, self.size).into_bytes()
    }
}

#[derive(Debug)]
pub enum Object {
    Blob(Blob),
    TreeNode(TreeNode),
}

impl Object {
    pub fn get_type(&self) -> &'static str {
        match self {
            Self::Blob(_) => "file",
            Self::TreeNode(_) => "tree",
        }
    }

    pub fn read(root: impl AsRef<Path>, hash: &str) -> Self {
        let compressed = input_output::read_obj(root, hash);
        let bytes = codec::decompress(&compressed);
        let mut reader = BytesReader::new(&bytes);
        let header = Header::parse(&mut reader);
        assert_eq!(reader.len(), header.size);

        match header.kind {
            "blob" => Self::Blob(Blob::parse(&mut reader)),
            "tree" => Self::TreeNode(TreeNode::parse(&mut reader)),
            kind => panic!("unknown object type: {}", kind),
        }
    }

    pub fn write(&self, root: impl AsRef<Path>) -> Hash {
        let (hash, encoded) = match self {
            Self::Blob(blob) => blob.encode(),
            Self::TreeNode(_) => unimplemented!(),
        };
        input_output::write_obj(root, &hash.to_string(), &encoded);
        hash
    }

    #[cfg(test)]
    pub fn as_blob(self) -> Blob {
        match self {
            Self::Blob(blob) => blob,
            _ => panic!("not blob"),
        }
    }

    #[cfg(test)]
    pub fn as_tree(self) -> TreeNode {
        match self {
            Self::TreeNode(tree) => tree,
            _ => panic!("not tree"),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::{blob::Blob, input_output, object::Object, reference_impl, repo::Repo, test_utils};

    #[test]
    fn test_read_blob() {
        let root = test_utils::create_test_dir();
        let repository = reference_impl::create_repository(&root);

        let filename = "hello.txt";
        let contents = "Hello World!";
        fs::write(root.join(filename), contents).unwrap();
        let hash = reference_impl::git_add_path(&repository, filename);

        let blob = Object::read(root, &hash).as_blob();
        assert_eq!(blob.content, contents.as_bytes());
    }

    #[test]
    fn test_write_blob() {
        let filename = "hello.txt";
        let contents = "Hello World!";

        // want
        let root = test_utils::create_test_dir();
        let repository = reference_impl::create_repository(&root);
        fs::write(root.join(filename), contents).unwrap();
        let hash_want = reference_impl::git_add_path(&repository, filename);
        let encoded_want = input_output::read_obj(&root, &hash_want);

        // got
        let root = test_utils::create_test_dir();
        let repo = Repo::new(&root);
        repo.init();
        let blob = Blob::new(String::from(contents).bytes().collect());
        let obj = Object::Blob(blob);
        let hash_got = obj.write(&root);
        let encoded_got = input_output::read_obj(&root, &hash_got.to_string());

        assert_eq!(hash_got.to_string(), hash_want);
        assert_eq!(encoded_got, encoded_want);
    }

    #[test]
    fn test_read_tree() {
        let root = test_utils::create_test_dir();
        let repository = reference_impl::create_repository(&root);

        let contents = "";
        input_output::write(root.join("file1"), contents);
        input_output::write(root.join("dir1/file_in_dir_1"), contents);
        input_output::write(root.join("dir1/file_in_dir_2"), contents);
        input_output::write(root.join("dir2/file_in_dir_3"), contents);
        reference_impl::git_add_all(&repository);
        let hash = reference_impl::git_write_tree(&repository);

        let tree_node = Object::read(root, &hash).as_tree();
        let wants = ["dir1", "dir2", "file1"];
        for (got, want) in tree_node.into_iter().zip(wants) {
            assert_eq!(got.name, want)
        }
    }
}
