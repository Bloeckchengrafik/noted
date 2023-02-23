use lazy_static::lazy_static;

use bevy::prelude::*;

lazy_static!(
    pub static ref COLOR_PBLACK: Color = Color::hex("131B23").unwrap(); // Front background color
    pub static ref COLOR_RBLACK: Color = Color::hex("0D1821").unwrap(); // Back background color
    pub static ref COLOR_SAFRON: Color = Color::hex("E8C547").unwrap(); // Yellow
    pub static ref COLOR_SUBLUE: Color = Color::hex("6096BA").unwrap(); // Blue
    pub static ref COLOR_CBPINK: Color = Color::hex("EDAFB8").unwrap(); // Red
    pub static ref COLOR_JGREEN: Color = Color::hex("49A078").unwrap(); // Green
);