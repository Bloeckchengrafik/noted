use std::env::var;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
enum NodeType {
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct Node {
    node_type: NodeType,
    name: String,
    children: Vec<Node>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct FileTree {
    root: Node,
}

impl FileTree {
    pub fn new() -> Self {
        let mut data = Self {
            root: Node {
                node_type: NodeType::Directory,
                name: "".to_string(),
                children: Vec::new(),
            },
        };

        // Fill the tree
        Self::populate_subtree(&mut data.root, &Self::get_root_dir());

        data
    }

    fn populate_subtree(node: &mut Node, current_path: &String) {
        if node.node_type == NodeType::File {
            return;
        } else {
            //let path = current_path + &(FileTree::get_path_separator()) + &node.name;
            let mut path = String::new();
            path.push_str(current_path);
            path.push_str(&*FileTree::get_path_separator());
            path.push_str(&node.name);

            let mut directories: Vec<DirEntry> = Vec::new();
            let mut files: Vec<DirEntry> = Vec::new();

            for file_or_dir in fs::read_dir(Path::new(path.as_str())).unwrap() {
                let file_or_dir = file_or_dir.unwrap();
                let file_or_dir_type = file_or_dir.file_type().unwrap();

                if file_or_dir_type.is_dir() {
                    directories.push(file_or_dir);
                } else if file_or_dir_type.is_file() {
                    files.push(file_or_dir);
                }
            }

            for dir in directories {
                let mut new_node = Node {
                    node_type: NodeType::Directory,
                    name: dir.file_name().into_string().unwrap(),
                    children: Vec::new(),
                };

                Self::populate_subtree(&mut new_node, &path);

                node.children.push(new_node);
            }

            for file in files {
                let new_node = Node {
                    node_type: NodeType::File,
                    name: file.file_name().into_string().unwrap(),
                    children: Vec::new(),
                };

                node.children.push(new_node);
            }
        }
    }

    fn get_root_dir() -> String {
        let root_dir = if cfg!(target_os = "windows") {
            let homedir = var("USERPROFILE").unwrap();
            format!("{}\\.noted", homedir)
        } else {
            let homedir = var("HOME").unwrap();
            format!("{}/.noted", homedir)
        };

        if !Path::new(root_dir.as_str()).exists() {
            fs::create_dir_all(root_dir.as_str()).unwrap();
        }

        root_dir
    }

    fn get_path_separator() -> String {
        if cfg!(target_os = "windows") {
            "\\".to_string()
        } else {
            "/".to_string()
        }
    }
}
