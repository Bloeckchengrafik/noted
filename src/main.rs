use bevy::prelude::*;
use crate::input::InputPlugin;

mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputPlugin)
        .run();
}
