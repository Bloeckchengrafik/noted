use std::path::Path;
use crate::filetree;
use std::process::Command;

#[tauri::command]
pub fn get_filetree() -> filetree::FileTree {
    filetree::FileTree::new()
}

#[tauri::command]
pub fn get_base_path() -> String {
    filetree::FileTree::get_root_dir()
}

#[tauri::command]
pub fn rename(fqpn: String, new_name: String) -> bool {
    // Check if the new name is valid
    if new_name.contains("/") || new_name.contains("\\") {
        return false;
    }

    let root_dir = filetree::FileTree::get_root_dir();
    let basepath = Path::new(&root_dir);

    let path = basepath.join(Path::new(&fqpn));
    let new_path = path.with_file_name(new_name);

    // Check if the path is in the base directory
    if !path.starts_with(basepath) {
        println!("[!] Path is not in the base directory: {} / {}", path.display(), basepath.display());
        return false;
    }

    if path.exists() {
        std::fs::rename(path, new_path).is_ok()
    } else {
        false
    }
}

#[tauri::command]
pub fn delete(fqpn: String) -> bool {
    let root_dir = filetree::FileTree::get_root_dir();
    let base_path = Path::new(&root_dir);

    let path = base_path.join(Path::new(&fqpn));

    // Check if the path is in the base directory
    if !path.starts_with(base_path) {
        println!("[!] Path is not in the base directory");
        return false;
    }

    if path.exists() {
        if path.is_dir() {
            std::fs::remove_dir_all(path).is_ok()
        } else {
            std::fs::remove_file(path).is_ok()
        }
    } else {
        false
    }
}

#[tauri::command]
pub fn open_in_explorer(fqpn: String) -> bool {
    let root_dir = filetree::FileTree::get_root_dir();
    let base_path = Path::new(&root_dir);

    let path_with_filename = base_path.join(Path::new(&fqpn));
    let path_without_filename = path_with_filename.parent().unwrap();

    // Open the directory in explorer
    if cfg!(target_os = "windows") {
        Command::new("explorer")
            .arg(path_without_filename)
            .spawn()
            .is_ok()
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .arg(path_without_filename)
            .spawn()
            .is_ok()
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(path_without_filename)
            .spawn()
            .is_ok()
    } else {
        false
    }
}

#[tauri::command]
pub fn open_in_default_app(fqpn: String) -> bool {
    let path = Path::new(&fqpn);

    // Open the file in the default application
    opener::open(path).is_ok()
}