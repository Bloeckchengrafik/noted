use std::path::PathBuf;


fn get_user_settings_dir() -> Option<PathBuf> {
  dirs::config_dir().map(|mut p| {
    p.push("glyph");
    p
  })
}

/// Assume the location is not writable
fn get_system_settings_dir() -> Option<PathBuf> {
  #[cfg(target_os = "linux")]
  return Some(PathBuf::from("/etc/glyph"));
  #[cfg(target_os = "macos")]
  return None; // TODO figure this out
  #[cfg(target_os = "windows")]
  return Some(PathBuf::from("C:/ProgramData/glyph"));
}

pub fn get_settings_file_locations(kind: &str) -> Vec<PathBuf> {
  let mut path_options = vec![];
  if let Some(mut p) = std::env::current_dir().ok() {
    p.push(kind);
    path_options.push(p);
  }

  if let Some(mut p) = get_user_settings_dir() {
    p.push(kind);
    path_options.push(p);
  }

  if let Some(mut p) = get_system_settings_dir() {
    p.push(kind);
    path_options.push(p);
  }

  path_options.iter().filter(|p| p.exists()).cloned().collect()
}