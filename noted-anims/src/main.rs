mod gui;

use bevy::prelude::*;
use crate::gui::GuiPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GuiPlugin)
        .run();
}
