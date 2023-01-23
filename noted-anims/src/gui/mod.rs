pub(crate) mod theme;
mod timeline;
mod menubar;
mod resizable;

use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use crate::gui::menubar::build_menubar_ui;
use crate::gui::resizable::do_resize;
use crate::gui::theme::*;
use crate::gui::timeline::{build_timeline_ui, TimelineSettings};

const TIMELINE_HEIGHT: i32 = 200;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: COLOR_BACKGROUND.into(),
        ..Default::default()
    })
        .with_children(|parent|
            {

                build_menubar_ui(
                    parent,
                    &asset_server,
                );

                parent.spawn(
                    TextBundle::from_section(
                        "Text Example",
                        TextStyle {
                            font: asset_server.load("fonts/VictorMono-Medium.ttf"),
                            font_size: 30.0,
                            color: COLOR_TEXT.into(),
                        },
                    )
                        .with_style(
                            Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            }
                        )
                );

                build_timeline_ui(
                    parent,
                    &asset_server,
                    TimelineSettings {
                        height: TIMELINE_HEIGHT,
                    },
                )
            }
        );
}

pub(crate) struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app());
        app.add_startup_system(setup);
        app.add_system(do_resize);
    }
}