use std::{path::Path, str::from_utf8};

use crate::{blob::Blob, bytes_reader::BytesReader, codec, input_output};

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

    use crate::{object::Object, reference_impl, test_utils};

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
}
