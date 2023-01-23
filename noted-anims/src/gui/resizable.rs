use std::cmp::max;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub(crate) struct Resizable {
    pub height: f32,
    pub width: f32,
    pub min_height: f32,
    pub min_width: f32,
    pub resizing: Option<Bound>,
    pub allow_resize_in: Vec<Bound>,
}

impl Resizable {
    pub fn new(height: f32, width: f32, min_height: f32, min_width: f32, allow_resize_in: Vec<Bound>) -> Self {
        Self {
            height,
            width,
            min_height,
            min_width,
            resizing: None,
            allow_resize_in,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bound {
    is_vertical: bool,
    sign: i32,
}

impl Bound {
    pub fn top() -> Self {
        Self::new(true, 1)
    }

    pub fn bottom() -> Self {
        Self::new(true, -1)
    }

    pub fn left() -> Self {
        Self::new(false, -1)
    }

    pub fn right() -> Self {
        Self::new(false, 1)
    }

    pub fn new(is_vertical: bool, sign: i32) -> Self {
        Self { is_vertical, sign }
    }

    pub fn is_vertical(&self) -> bool {
        self.is_vertical
    }

    pub fn sign(&self) -> i32 {
        self.sign
    }

    pub fn delta(&self, window: &Window, transform: &GlobalTransform, resizable: &Mut<Resizable>) -> f32 {
        let mouse_position = if self.is_vertical {
            window.height() - window.cursor_position().unwrap().y
        } else {
            window.cursor_position().unwrap().x
        };

        let upper_edge = if self.is_vertical {
            transform.translation().y - (self.sign as f32 * resizable.height / 2.0)
        } else {
            transform.translation().x + (self.sign as f32 * resizable.width / 2.0)
        };

        info!("Mouse position: {}, upper edge: {}, delta: {}", mouse_position, upper_edge, (mouse_position - upper_edge).abs());

        (mouse_position - upper_edge).abs()
    }
}

pub(crate) fn do_resize(
    mut query: Query<(&mut Resizable, &mut Style, &GlobalTransform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().expect("No primary window found, but there should be one if there is a resizable element");

    for (mut resizable, mut style, transform) in query.iter_mut() {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            for bound in resizable.allow_resize_in.iter() {
                if bound.delta(&window, &transform, &resizable) < 10.0 {
                    resizable.resizing = Some(bound.clone());
                    break;
                }
            }
        }

        if mouse_button_input.pressed(MouseButton::Left) {
            if resizable.resizing.is_none() {
                info!("Mouse button released, but no resizing was in progress");
                continue;
            }

            for motion_event in mouse_motion_events.iter() {
                let delta = motion_event.delta;

                if resizable.resizing.unwrap().is_vertical() {
                    resizable.height -= delta.y * resizable.resizing.unwrap().sign() as f32;
                    if resizable.height < resizable.min_height {
                        resizable.height = resizable.min_height;
                    }

                    style.size.height = Val::Px(resizable.height);
                } else {
                    resizable.width += delta.x * resizable.resizing.unwrap().sign() as f32;
                    if resizable.width < resizable.min_width {
                        resizable.width = resizable.min_width;
                    }

                    style.size.width = Val::Px(resizable.width);
                }
            }
        } else {
            resizable.resizing = None;
        }
    }
}