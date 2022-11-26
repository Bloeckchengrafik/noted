use crate::saveload;

#[tauri::command]
pub fn get_settings() -> saveload::Settings {
    saveload::SavedData::from_file().settings
}

#[tauri::command]
pub fn save_settings(settings: saveload::Settings) {
    let mut data = saveload::SavedData::from_file();
    data.settings = settings;
    data.save();
}
