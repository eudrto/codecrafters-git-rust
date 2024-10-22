use std::path::Path;

use git2::{IndexAddOption, Repository};

pub fn create_repository(root: impl AsRef<Path>) -> Repository {
    Repository::init(&root).unwrap()
}

pub fn git_add_path(repo: &Repository, path: &str) -> String {
    let mut index = repo.index().unwrap();
    index.add_path(Path::new(path)).unwrap();
    let oid = index.get_path(Path::new(path), 0).unwrap().id;
    oid.to_string()
}

pub fn git_add_all(repo: &Repository) {
    let mut index = repo.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
}

pub fn git_write_tree(repo: &Repository) -> String {
    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    oid.to_string()
}
