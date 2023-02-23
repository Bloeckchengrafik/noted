use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use crate::input::input_device::*;

pub struct LinuxInputDevice;

impl InputDevice for LinuxInputDevice {
    fn new() -> Self {
        LinuxInputDevice
    }

    fn add_to_app(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, input_device_onoff_system);
        app.add_system_to_stage(CoreStage::PreUpdate, input_device_move_system);
    }
}

fn input_device_onoff_system(buttons: Res<Input<MouseButton>>, mut ev_up: EventWriter<StylusInputDeviceUpEvent>, mut ev_down: EventWriter<StylusInputDeviceDownEvent>) {
    if buttons.just_released(MouseButton::Left) {
        ev_up.send(StylusInputDeviceUpEvent);
    }

    if buttons.just_pressed(MouseButton::Left) {
        ev_down.send(StylusInputDeviceDownEvent);
    }
}

fn input_device_move_system(
    mut ev: EventWriter<StylusInputDeviceEvent>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
) {
    for event in mouse_motion_event_reader.iter() {
        ev.send(StylusInputDeviceEvent {
            x: event.delta.x,
            y: event.delta.y,
            pressure: 1.0,
        });
    }
}