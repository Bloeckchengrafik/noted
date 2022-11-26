use std::env::var;
use std::fs;
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

        println!("Root dir: {:?}", Self::get_root_dir());

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

            for file in fs::read_dir(Path::new(path.as_str())).unwrap() {
                let file = file.unwrap();

                let file_name = file.file_name().into_string().unwrap();
                let file_type = file.metadata().unwrap().file_type();
                let node_type = if file_type.is_file() {
                    NodeType::File
                } else {
                    NodeType::Directory
                };
                let mut new_node = Node {
                    node_type,
                    name: file_name.to_string(),
                    children: Vec::new(),
                };
                Self::populate_subtree(&mut new_node, &path);
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
