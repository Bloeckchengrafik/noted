use bevy::app::App;
use bevy::prelude::*;

pub trait InputDevice {
    fn new() -> Self;
    fn add_to_app(&self, app: &mut App);
}

pub struct StylusInputDeviceEvent {
    pub x: f32,
    pub y: f32,
    pub pressure: f32, // 0.0 - 1.0
}

pub struct StylusInputDeviceDownEvent;

pub struct StylusInputDeviceUpEvent;
