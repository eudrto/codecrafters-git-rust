use std::{fs, path::Path};

use crate::{
    blob::Blob,
    hash::Hash,
    input_output,
    object::Object,
    tree_node::{TreeNode, TreeNodeEntry},
};

fn build_tree(path: impl AsRef<Path>) -> Vec<Object> {
    let mut tree = vec![];
    let mut children = vec![];
    for path in input_output::read_dir_sorted(path) {
        dbg!(&path);
        let tree_node_entry = if path.is_dir() {
            if is_dot_git(&path) {
                continue;
            }
            tree.append(&mut build_tree(&path));
            let mode = String::from("40000");
            let name = input_output::basename(&path);
            let hash = tree.last().unwrap().hash();
            TreeNodeEntry::new(mode, name, hash)
        } else {
            let content = fs::read(&path).unwrap();
            let obj = Object::Blob(Blob::new(content));
            let hash = obj.hash();
            tree.push(obj);

            let mode = format!("{:o}", input_output::get_mode(&path));
            let name = input_output::basename(&path);
            TreeNodeEntry::new(mode, name, hash)
        };
        children.push(tree_node_entry);
    }
    tree.push(Object::TreeNode(TreeNode::new(children)));
    tree
}

pub fn write_tree(root: impl AsRef<Path>) -> Option<Hash> {
    let mut tree = build_tree(&root);
    let Some(root_node) = tree.pop() else {
        return None;
    };
    for obj in tree {
        obj.write(&root);
    }
    Some(root_node.write(&root))
}

fn is_dot_git(path: impl AsRef<Path>) -> bool {
    input_output::basename(path) == ".git"
}

#[cfg(test)]
mod tests {
    use crate::{
        input_output, reference_impl,
        repo::Repo,
        test_utils,
        tree::{build_tree, write_tree},
    };

    #[test]
    fn test_build_tree() {
        let root = test_utils::create_test_dir();

        let contents = "";
        input_output::write(root.join("file1"), contents);
        input_output::write(root.join("dir1/file_in_dir_1"), contents);
        input_output::write(root.join("dir1/file_in_dir_2"), contents);
        input_output::write(root.join("dir2/file_in_dir_3"), contents);

        let tree = build_tree(&root);
        assert_eq!(tree.len(), 7);

        assert_eq!(
            tree.iter().filter(|obj| obj.get_type() == "file").count(),
            4
        );
        assert_eq!(
            tree.iter().filter(|obj| obj.get_type() == "tree").count(),
            3
        );
    }

    #[test]
    fn test_write_tree() {
        // want
        let root = test_utils::create_test_dir();
        let repository = reference_impl::create_repository(&root);

        let contents = "";
        input_output::write(root.join("file1"), contents);
        input_output::write(root.join("dir1/file_in_dir_1"), contents);
        input_output::write(root.join("dir1/file_in_dir_2"), contents);
        input_output::write(root.join("dir2/file_in_dir_3"), contents);
        reference_impl::git_add_all(&repository);
        let hash_want = reference_impl::git_write_tree(&repository);

        // got
        let root = test_utils::create_test_dir();
        let repo = Repo::new(&root);
        repo.init();

        let contents = "";
        input_output::write(root.join("file1"), contents);
        input_output::write(root.join("dir1/file_in_dir_1"), contents);
        input_output::write(root.join("dir1/file_in_dir_2"), contents);
        input_output::write(root.join("dir2/file_in_dir_3"), contents);
        let hash_got = write_tree(root).unwrap();

        assert_eq!(hash_got.to_string(), hash_want);
    }
}
