use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::{blob::Blob, input_output, object::Object, tree};

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
            Object::TreeNode(_) => self.ls_tree(false, hash),
        };
    }

    pub fn hash_object(&self, path: &str) {
        let content = fs::read(self.root.join(path)).unwrap();
        let obj = Object::Blob(Blob::new(content));
        let hash = obj.write(self.get_root());
        print!("{}", hash);
    }

    pub fn ls_tree(&self, name_only: bool, tree_ish: &str) {
        let Object::TreeNode(tree) = Object::read(self.get_root(), tree_ish) else {
            panic!("fatal: not a tree object")
        };

        if name_only {
            tree.into_iter()
                .map(|entry| &entry.name)
                .for_each(|name| println!("{}", name));
            return;
        }

        for entry in &tree {
            let obj = Object::read(self.get_root(), &entry.hash.to_string());
            println!(
                "{:0>6} {} {}\t{}",
                entry.mode,
                obj.get_type(),
                entry.hash,
                entry.name
            )
        }
    }

    pub fn write_tree(&self) {
        let hash = tree::write_tree(self.get_root());
        if let Some(hash) = hash {
            print!("{}", hash)
        }
    }
}
