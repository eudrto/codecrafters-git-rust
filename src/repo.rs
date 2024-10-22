use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::{input_output, object::Object};

pub struct Repo {
    root: PathBuf,
}

impl Repo {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn new_current_dir() -> Self {
        Self::new(env::current_dir().unwrap())
    }

    pub fn get_root(&self) -> &Path {
        &self.root
    }

    pub fn init(&self) {
        fs::create_dir_all(self.get_root().join(".git/objects")).unwrap();
        fs::create_dir_all(self.get_root().join(".git/refs")).unwrap();
        input_output::write(self.get_root().join(".git/HEAD"), "ref: refs/heads/main\n");
    }

    pub fn cat_file(&self, hash: &str) {
        let obj = Object::read(self.get_root(), hash);
        match obj {
            Object::Blob(blob) => print!("{}", blob),
        };
    }
}
