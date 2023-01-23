use bevy::prelude::*;
use crate::gui::resizable::{Bound, Resizable};
use crate::gui::theme::COLOR_SECOND_BACKGROUND;

pub(crate) struct TimelineSettings {
    pub(crate) height: i32,
}

pub(crate) fn build_timeline_ui(
    parent: &mut ChildBuilder,
    _asset_server: &Res<AssetServer>,
    settings: TimelineSettings
) {
    // create a box representing the timeline
    parent.spawn(
        (NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(settings.height as f32)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            background_color: COLOR_SECOND_BACKGROUND.into(),
            ..Default::default()
        }, Resizable::new(
            settings.height as f32,
            0.0,
            10.0,
            0.0,
            vec![Bound::top()]
        ))
    );
}