mod keymap;
mod files;

use gpui::{App, Window};

pub fn init(window: &mut Window, cx: &mut App) {
  keymap::init(window, cx);
}
