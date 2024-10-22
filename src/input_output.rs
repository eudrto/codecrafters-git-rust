use std::{
    fs,
    os::unix::fs::PermissionsExt,
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

pub fn write_obj(root: impl AsRef<Path>, hash: &str, contents: &[u8]) {
    write(get_obj_path(root, hash), contents);
}

pub fn read_dir_sorted(path: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut paths: Vec<_> = fs::read_dir(path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    paths
}

pub fn basename(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_string_lossy()
        .into_owned()
}

pub fn get_mode(path: impl AsRef<Path>) -> u32 {
    let metadata = fs::symlink_metadata(&path).unwrap();
    if metadata.is_symlink() {
        0o120000
    } else if is_executable(&path) {
        0o100755
    } else {
        0o100644
    }
}

fn is_executable(path: impl AsRef<Path>) -> bool {
    fs::symlink_metadata(path).unwrap().permissions().mode() & 0o100 != 0
}

#[cfg(test)]
mod tests {
    use std::{fs::File, os::unix::fs::PermissionsExt, path::Path};

    use crate::{input_output::basename, test_utils};

    use super::is_executable;

    #[test]
    fn test_basename() {
        assert_eq!(basename(Path::new("dir1/dir2/file.txt")), "file.txt");
        assert_eq!(basename(Path::new("dir1/dir2/dir3")), "dir3");
        assert_eq!(basename(Path::new("dir1/dir2/dir3/")), "dir3");
    }

    #[test]
    fn test_is_executable_no() {
        let root = test_utils::create_test_dir();
        let path = root.join("file");
        let _f = File::create(&path).unwrap();
        // println!("{:o}", _f.metadata().unwrap().permissions().mode());
        assert!(!is_executable(&path));
    }

    #[test]
    fn test_is_executable_yes() {
        let root = test_utils::create_test_dir();
        let path = root.join("file");
        let f = File::create(&path).unwrap();
        let mut p = f.metadata().unwrap().permissions();
        p.set_mode(p.mode() | 0o100);
        f.set_permissions(p).unwrap();
        // println!("{:o}", f.metadata().unwrap().permissions().mode());
        assert!(is_executable(&path));
    }
}
