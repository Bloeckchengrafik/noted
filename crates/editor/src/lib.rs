/*
 * Editor heavily based on https://github.com/longbridge/gpui-component licensed under Apache-2.0
 * See legal/LICENSE-GPUI-COMPONENTS for more information
 */
mod actions;
pub mod editor;
mod element;
mod cursor;
mod markdown;

pub fn init(cx: &mut gpui::App) {
    actions::init(cx);
}