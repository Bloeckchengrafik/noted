use bevy::prelude::*;
use bevy::winit::WinitSettings;
use crate::input::InputPlugin;

mod input;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(InputPlugin)
        .add_startup_system(ui::setup)
        .run();
}
