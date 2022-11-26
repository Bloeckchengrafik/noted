#[tauri::command]
pub(crate) fn current_user() -> String {
    whoami::username()
}