#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod filetree;
mod saveload;
mod commands;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::user::current_user,
            commands::tree::get_filetree,
            commands::tree::get_base_path,
            commands::tree::rename,
            commands::tree::delete,
            commands::tree::open_in_explorer,
            commands::tree::open_in_default_app,
            commands::saveload::get_settings,
            commands::saveload::save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
