use std::{path::Path, str::from_utf8};

use crate::{blob::Blob, bytes_reader::BytesReader, codec, hash::Hash, input_output};

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
}

impl Object {
    pub fn read(root: impl AsRef<Path>, hash: &str) -> Self {
        let compressed = input_output::read_obj(root, hash);
        let bytes = codec::decompress(&compressed);
        let mut reader = BytesReader::new(&bytes);
        let header = Header::parse(&mut reader);
        assert_eq!(reader.len(), header.size);

        match header.kind {
            "blob" => Self::Blob(Blob::parse(&mut reader)),
            kind => panic!("unknown object type: {}", kind),
        }
    }

    pub fn write(&self, root: impl AsRef<Path>) -> Hash {
        let (hash, encoded) = match self {
            Self::Blob(blob) => blob.encode(),
        };
        input_output::write_obj(root, &hash.to_string(), &encoded);
        hash
    }

    #[cfg(test)]
    pub fn as_blob(self) -> Blob {
        match self {
            Self::Blob(blob) => blob,
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
}
