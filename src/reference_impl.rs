use std::path::Path;

use git2::Repository;

pub fn create_repository(root: impl AsRef<Path>) -> Repository {
    Repository::init(&root).unwrap()
}

pub fn git_add_path(repo: &Repository, path: &str) -> String {
    let mut index = repo.index().unwrap();
    index.add_path(Path::new(path)).unwrap();
    let oid = index.get_path(Path::new(path), 0).unwrap().id;
    oid.to_string()
}
