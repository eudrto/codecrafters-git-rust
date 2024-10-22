use std::{
    fs,
    path::{Path, PathBuf},
};

fn get_test_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let pkg_name = env!("CARGO_PKG_NAME");
    Path::new(manifest_dir)
        .parent()
        .unwrap()
        .join(format!("{}-tests", pkg_name))
}

pub fn create_test_dir() -> PathBuf {
    let root = get_test_dir();
    let _ = fs::remove_dir_all(&root);
    fs::create_dir(&root).unwrap();
    root
}
