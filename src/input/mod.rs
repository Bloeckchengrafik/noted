mod linux_input_device;
pub mod input_device;

use bevy::prelude::*;
use crate::input::input_device::{InputDevice, StylusInputDeviceDownEvent, StylusInputDeviceEvent, StylusInputDeviceUpEvent};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {

        app.add_event::<StylusInputDeviceEvent>();
        app.add_event::<StylusInputDeviceDownEvent>();
        app.add_event::<StylusInputDeviceUpEvent>();

        // Detect the platform and create the appropriate input device
        if cfg!(unix) {
            let input_device = linux_input_device::LinuxInputDevice::new();
            input_device.add_to_app(app);
        } else {
            panic!("Unsupported platform");
        }
    }
}