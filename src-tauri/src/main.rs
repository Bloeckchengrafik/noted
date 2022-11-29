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
            commands::tree::create_file,
            commands::tree::create_dir,
            commands::tree::open_dir_in_explorer,
            commands::tree::open_dir_in_default_terminal,
            commands::tree::rename_dir,
            commands::tree::delete_dir,
            commands::saveload::get_settings,
            commands::saveload::save_settings,
            commands::file_contents::file_contents,
            commands::file_contents::write_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
