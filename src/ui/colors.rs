use lazy_static::lazy_static;

use bevy::prelude::*;

lazy_static!(
    pub static ref COLOR_BACKGROUND: Color = Color::hex("131317").unwrap();
    pub static ref COLOR_SURFACE: Color    = Color::hex("212029").unwrap();
    pub static ref COLOR_GRAY: Color       = Color::hex("B1B1B1").unwrap();
    pub static ref COLOR_WHITE: Color      = Color::hex("FFFFFF").unwrap();
);