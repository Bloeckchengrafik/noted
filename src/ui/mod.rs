pub mod colors;

use bevy::prelude::*;
use crate::ui::colors::COLOR_SURFACE;

pub fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Add a bar at the left of the screen
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(50.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor::from(*COLOR_SURFACE),
                ..default()
            });
        });
}