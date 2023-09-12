#[cfg(target_os = "linux")]
mod libinput_input_device;

pub mod input_device;

use std::sync::Mutex;
use bevy::prelude::*;
use crate::input::input_device::{StylusInputDeviceManager, StylusInputDeviceDownEvent, StylusInputDeviceEvent, StylusInputDeviceUpEvent};

pub struct InternalInputData {
    x: f32,
    y: f32,
    pressure: f32,
    down: bool,
}

static INTERNAL_INPUT_DATA: Mutex<InternalInputData> = Mutex::new(InternalInputData {
    x: 0.0,
    y: 0.0,
    pressure: 0.0,
    down: false,
});

#[cfg(target_os = "linux")]
fn get_device() -> Option<Box<libinput_input_device::LibInputInputDevice>> {
    Some(Box::new(libinput_input_device::LibInputInputDevice::new()))
}

#[cfg(not(target_os = "linux"))]
fn get_device() -> Option<Option<Box<dyn StylusInputDeviceManager>>> {
    None
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StylusInputDeviceEvent>();
        app.add_event::<StylusInputDeviceDownEvent>();
        app.add_event::<StylusInputDeviceUpEvent>();


        let mut input_device: Option<Box<dyn StylusInputDeviceManager>> = get_device();
        if let Some(input_device) = input_device {
            input_device.add_to_app(app);
        } else {
            panic!("No input device found");
        }

        app.add_systems(PreUpdate, event_creator);
    }
}

static LAST_DOWN: Mutex<bool> = Mutex::new(false);
static LAST_X: Mutex<f32> = Mutex::new(0.0);
static LAST_Y: Mutex<f32> = Mutex::new(0.0);
static LAST_PRESSURE: Mutex<f32> = Mutex::new(0.0);

fn event_creator(
    mut ev: EventWriter<StylusInputDeviceEvent>,
    mut ev_down: EventWriter<StylusInputDeviceDownEvent>,
    mut ev_up: EventWriter<StylusInputDeviceUpEvent>,
) {
    let data = INTERNAL_INPUT_DATA.lock().unwrap();
    let mut last_down = LAST_DOWN.lock().unwrap();
    let mut last_x = LAST_X.lock().unwrap();
    let mut last_y = LAST_Y.lock().unwrap();
    let mut last_pressure = LAST_PRESSURE.lock().unwrap();

    if data.down && !*last_down {
        ev_down.send(StylusInputDeviceDownEvent);
    } else if !data.down && *last_down {
        ev_up.send(StylusInputDeviceUpEvent);
    }

    if data.x != *last_x || data.y != *last_y || data.pressure != *last_pressure {
        ev.send(StylusInputDeviceEvent {
            x: data.x,
            y: data.y,
            pressure: data.pressure,
        });
    }

    *last_down = data.down;
    *last_x = data.x;
    *last_y = data.y;
    *last_pressure = data.pressure;
}
