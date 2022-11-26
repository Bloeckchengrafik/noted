#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod filetree;
mod saveload;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn current_user() -> String {
    whoami::username()
}

#[tauri::command]
fn get_filetree() -> filetree::FileTree {
    filetree::FileTree::new()
}

#[tauri::command]
fn get_settings() -> saveload::Settings {
    saveload::SavedData::from_file().settings
}

#[tauri::command]
fn save_settings(settings: saveload::Settings) {
    let mut data = saveload::SavedData::from_file();
    data.settings = settings;
    data.save();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![current_user, get_filetree, get_settings, save_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
