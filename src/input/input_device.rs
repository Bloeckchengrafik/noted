use bevy::app::App;
use bevy::prelude::Event;

pub trait StylusInputDeviceManager {
    fn add_to_app(&self, app: &mut App);
}

pub struct StylusInputDeviceEvent {
    pub x: f32,
    pub y: f32,
    pub pressure: f32, // 0.0 - 1.0
}

impl Event for StylusInputDeviceEvent {}

pub struct StylusInputDeviceDownEvent;
impl Event for StylusInputDeviceDownEvent {}

pub struct StylusInputDeviceUpEvent;
impl Event for StylusInputDeviceUpEvent {}
