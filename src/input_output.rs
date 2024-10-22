use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap()
}

fn get_obj_path(root: impl AsRef<Path>, hash: &str) -> PathBuf {
    root.as_ref()
        .join(".git")
        .join("objects")
        .join(&hash[..2])
        .join(&hash[2..])
}

pub fn read_obj(root: impl AsRef<Path>, hash: &str) -> Vec<u8> {
    fs::read(get_obj_path(root, hash)).unwrap()
}
