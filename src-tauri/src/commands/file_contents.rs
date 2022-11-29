use std::io::Write;

#[tauri::command]
pub fn file_contents(path: String) -> String {
    let basepath_str = crate::filetree::FileTree::get_root_dir();
    let path = format!("{}{}", basepath_str, path);
    println!("Getting File Contents: {}", path);
    std::fs::read_to_string(path).unwrap()
}

#[tauri::command]
pub fn write_file(path: String, content: String) -> bool {
    let basepath_str = crate::filetree::FileTree::get_root_dir();
    let mut path = format!("{}{}", basepath_str, path);
    path = path.replace("/", std::path::MAIN_SEPARATOR.to_string().as_str());
    println!("Writing File Contents: {}", path);
    let path_obj = std::path::Path::new(&path);
    let mut file = std::fs::File::create(path_obj).unwrap();
    println!("Writing to: {:?}", path_obj);
    let write = write!(&mut file, "{}", content).is_ok();
    write && file.flush().is_ok()
}