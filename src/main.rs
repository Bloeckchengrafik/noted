#![feature(stmt_expr_attributes)]

use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy::winit::WinitSettings;
use bevy_winit::WinitPlugin;
use crate::input::InputPlugin;
use crate::inspect::InspectPlugin;

mod input;
mod ui;
mod inspect;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Noted {}", VERSION),
                ..Default::default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        }).set(WinitPlugin))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins((InputPlugin, InspectPlugin))
        .add_systems(Startup, ui::setup)
        .run();
}
