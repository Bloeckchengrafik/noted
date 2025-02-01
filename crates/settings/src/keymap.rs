use crate::files::get_settings_file_locations;
use anyhow::Result;
use components::notification::Notification;
use components::ContextModal;
use gpui::{App, KeyBinding, KeyBindingContextPredicate};

#[cfg(any(target_os = "linux",  target_os = "windows"))]
pub const DEFAULT_KEYMAP: &str = include_str!("../keymaps/linux-windows.toml");

#[cfg(not(any(target_os = "linux",  target_os = "windows")))]
compile_error!("Unsupported OS, please open an issue on GitHub");

fn build_keybinding(cx: &mut App, key: String, action: String, context: Option<String>) -> Result<KeyBinding> {
  let action = cx.build_action(&action, None)?; // TODO handle data on actions
  let context_predicate = if let Some(context) = context {
    Some(KeyBindingContextPredicate::parse(&context)?.into())
  } else {
    None
  };
  Ok(KeyBinding::load(&key, action, context_predicate, None)?)
}

fn load_contextualized(window: &mut gpui::Window, cx: &mut App, context: &str, table: &toml::Table) {
  let mut items = vec![];

  for (key, value) in table {
    if value.is_str() {
      let action = value.as_str().unwrap();
      let binding = build_keybinding(cx, key.to_string(), action.to_string(), Some(context.to_string()));
      if let Ok(binding) = binding {
        items.push(binding);
      } else {
        window.push_notification(
          Notification::error(&format!("Failed to build contextualized keybinding for key {}: {}", key, binding.err().unwrap()))
            .autohide(true),
          cx,
        );
      }
    }
  }

  cx.bind_keys(items);
}

fn load_keymap(window: &mut gpui::Window, cx: &mut App, keymap: &str) {
  let Ok(table) = toml::from_str::<toml::Table>(keymap) else {
    window.push_notification(
      Notification::error("Failed to parse keymap file")
        .autohide(true),
      cx,
    );
    return;
  };

  for (key, value) in &table {
    if value.is_table() {
      load_contextualized(window, cx, &key, value.as_table().unwrap());
    } else if value.is_str() {
      let action = value.as_str().unwrap();
      let binding = build_keybinding(cx, key.to_string(), action.to_string(), None);
      if let Ok(binding) = binding {
        cx.bind_keys(vec![binding]);
      } else {
        window.push_notification(
          Notification::error(&format!("Failed to build uncontextualized keybinding for key {}: {}", key, binding.err().unwrap()))
            .autohide(true),
          cx,
        );
      }
    } else {
      window.push_notification(
        Notification::error(&format!("Invalid keybinding: {}", key))
          .autohide(true),
        cx,
      );
    }
  }
}

pub fn reload_keymap_tree(window: &mut gpui::Window, cx: &mut App, announce: bool) {
  cx.clear_key_bindings();
  load_keymap(window, cx, DEFAULT_KEYMAP);

  get_settings_file_locations("keymap.toml").iter().for_each(|path| {
    if let Ok(keymap) = std::fs::read_to_string(path) {
      load_keymap(window, cx, &keymap);
    }
  });

  if announce {
    window.push_notification(
      Notification::info("Keymap reloaded")
        .autohide(true),
      cx,
    );
  }
}

pub fn init(window: &mut gpui::Window, cx: &mut App) {
  reload_keymap_tree(window, cx, false);
}

