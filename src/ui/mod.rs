pub mod colors;

use bevy::prelude::*;
use crate::ui::colors::{COLOR_PBLACK, COLOR_RBLACK};

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
            background_color: BackgroundColor::from(*COLOR_PBLACK),
            ..default()
        })
        .with_children(|parent| {
            // Add a bar at the top of the screen
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    ..default()
                },
                background_color: BackgroundColor::from(*COLOR_RBLACK),
                ..default()
            });
        });
}