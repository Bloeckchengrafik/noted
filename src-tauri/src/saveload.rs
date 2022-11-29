use std::env::var;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// The automatic state-save
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub file_tree_open: bool,
    pub opened_dirs: Vec<String>,
    pub opened_files: Vec<String>,
    pub editor_font_size: u32,
}

/// The Preferences struct is used to store the user's preferences. It is
/// not for automatic state saving.
#[derive(Serialize, Deserialize)]
pub struct Preferences {
}

#[derive(Serialize,Deserialize)]
pub(crate) struct SavedData {
    pub settings: Settings,
    pub preferences: Preferences,
}

impl SavedData {
    fn new() -> Self {
        Self {
            settings: Settings {
                file_tree_open: true,
                opened_dirs: vec!["/".to_string()],
                opened_files: vec![],
                editor_font_size: 14,
            },
            preferences: Preferences {},
        }
    }

    pub(crate) fn from_file() -> Self {
        let data_dir = Self::get_data_dir();

        let path = Path::new(data_dir.as_str()).join(Path::new("data.json"));

        if path.exists() {
            let data = fs::read_to_string(path).unwrap();
            serde_json::from_str(data.as_str()).unwrap()
        } else {
            Self::new()
        }
    }

    fn get_data_dir() -> String {
        let root_dir = if cfg!(target_os = "windows") {
            let homedir = var("USERPROFILE").unwrap();
            format!("{}\\.noted\\.metadata", homedir)
        } else {
            let homedir = var("HOME").unwrap();
            format!("{}/.noted/.metadata", homedir)
        };

        if !Path::new(root_dir.as_str()).exists() {
            fs::create_dir_all(root_dir.as_str()).unwrap();
        }

        root_dir
    }

    pub(crate) fn save(&self) {
        let data_dir = Self::get_data_dir();
        let path = Path::new(data_dir.as_str()).join(Path::new("data.json"));
        let data = serde_json::to_string(self).unwrap();
        fs::write(path, data).unwrap();
    }
}
