use bevy::prelude::*;
use crate::gui::theme::*;

pub(crate) fn build_menubar_ui(
    parent: &mut ChildBuilder,
    _asset_server: &Res<AssetServer>,
) {
    // create a box representing the timeline
    parent.spawn(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(20 as f32)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            background_color: COLOR_BORDER.into(),
            ..Default::default()
        }
    );
}