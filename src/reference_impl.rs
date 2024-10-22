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

pub fn read_commit(root: impl AsRef<Path>, hash: &str) -> (String, Vec<String>, Option<String>) {
    let repo = Repository::open(root).unwrap();
    let oid = repo.revparse_single(hash).unwrap().id();
    let commit = repo.find_commit(oid).unwrap();

    let tree = commit.tree().unwrap().id().to_string();
    let parents: Vec<_> = commit
        .parents()
        .map(|commit| commit.id().to_string())
        .collect();
    let message = commit.message();
    (tree, parents, message.map(|m| m.to_string()))
}
