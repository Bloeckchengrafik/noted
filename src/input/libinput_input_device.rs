use std::fs::{File, OpenOptions};
use std::os::fd::OwnedFd;
use std::os::unix::prelude::OpenOptionsExt;
use std::path::Path;
use std::sync::Mutex;
use std::thread;
use bevy::prelude::*;
use input::{Libinput, LibinputInterface};
use input::Event::Tablet;
use input::event::tablet_tool::{TabletToolAxisEvent, TabletToolEventTrait};
use input::event::TabletToolEvent::Axis;
use crate::input::input_device::*;

extern crate libc;

use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use users::os::unix::GroupExt;
use crate::input::INTERNAL_INPUT_DATA;

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd)); // Modified from example for better code readability
    }
}

struct LinuxState {
    pressure: f64,
    down: bool,
}

static STATE: Mutex<LinuxState> = Mutex::new(LinuxState{
    pressure: 0.0,
    down: false,
});

pub struct LibInputInputDevice;

impl LibInputInputDevice {
    pub fn new() -> Self {
        Self
    }
}

impl StylusInputDeviceManager for LibInputInputDevice {
    fn add_to_app(&self, app: &mut App) {
        app.add_systems(PostStartup, controller_init);
        app.add_systems(PostUpdate, window_update);
    }
}

fn controller_init() {
    // Check if the user is in the input group
    let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
    let group = users::get_group_by_name("input").unwrap();
    let user_in_group = group.members().iter().map(|x| x.to_str().unwrap()).any(|x| x == user.name());
    if !user_in_group {
        dialog_box::warning("User is not in the input group. Stylus Input will not work.");
    }

    thread::spawn(input_loop);
}

fn window_update(
    mut motion_evr: EventReader<CursorMoved>,
) {
    for event in motion_evr.iter() {
        let Vec2{x, y}: Vec2 = event.position;
        let state_locked = STATE.lock().unwrap();
        let mut global_state = INTERNAL_INPUT_DATA.lock().unwrap();

        global_state.x = x;
        global_state.y = y;
        global_state.pressure = state_locked.pressure as f32;
        global_state.down = state_locked.down;
    }
}

fn input_loop() {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    info!("Starting to listen to udev events");
    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            if let Tablet(Axis(event)) = event {
                handle_tablet_event(event);
            }
        }
    }
}

fn handle_tablet_event(event: TabletToolAxisEvent) {
    let pressure = event.pressure();
    let down = pressure > 0.0;

    let mut state_locked = STATE.lock().unwrap();
    state_locked.pressure = pressure;
    state_locked.down = down;

    drop(state_locked); // Unlock the mutex
}