use bevy::prelude::Color;
use bevy::ui::BackgroundColor;

pub struct NotedColor(&'static str);

#[allow(dead_code)]
pub const COLOR_BACKGROUND: NotedColor = NotedColor("292D3E");

#[allow(dead_code)]
pub const COLOR_FOREGROUND: NotedColor = NotedColor("A6ACCD");

#[allow(dead_code)]
pub const COLOR_TEXT: NotedColor = NotedColor("676E95");

#[allow(dead_code)]
pub const COLOR_SELECTION_FOREGROUND: NotedColor = NotedColor("717CB470");

#[allow(dead_code)]
pub const COLOR_SELECTION_BACKGROUND: NotedColor = NotedColor("FFFFFF");

#[allow(dead_code)]
pub const COLOR_BUTTONS: NotedColor = NotedColor("303348");

#[allow(dead_code)]
pub const COLOR_SECOND_BACKGROUND: NotedColor = NotedColor("34324a");

#[allow(dead_code)]
pub const COLOR_DISABLED: NotedColor = NotedColor("515772");

#[allow(dead_code)]
pub const COLOR_CONTRAST: NotedColor = NotedColor("202331");

#[allow(dead_code)]
pub const COLOR_ACTIVE: NotedColor = NotedColor("414863");

#[allow(dead_code)]
pub const COLOR_BORDER: NotedColor = NotedColor("2b2a3e");

#[allow(dead_code)]
pub const COLOR_HIGHLIGHT: NotedColor = NotedColor("444267");

#[allow(dead_code)]
pub const COLOR_TREE: NotedColor = NotedColor("676E95");

#[allow(dead_code)]
pub const COLOR_NOTIFICATION: NotedColor = NotedColor("202331");

#[allow(dead_code)]
pub const COLOR_ACCENT: NotedColor = NotedColor("ab47bc");

#[allow(dead_code)]
pub const COLOR_EXCLUDED: NotedColor = NotedColor("2f2e43");

#[allow(dead_code)]
pub const COLOR_GREEN: NotedColor = NotedColor("c3e88d");

#[allow(dead_code)]
pub const COLOR_YELLOW: NotedColor = NotedColor("ffcb6b");

#[allow(dead_code)]
pub const COLOR_BLUE: NotedColor = NotedColor("82aaff");

#[allow(dead_code)]
pub const COLOR_RED: NotedColor = NotedColor("f07178");

#[allow(dead_code)]
pub const COLOR_PURPLE: NotedColor = NotedColor("c792ea");

#[allow(dead_code)]
pub const COLOR_ORANGE: NotedColor = NotedColor("f78c6c");

#[allow(dead_code)]
pub const COLOR_CYAN: NotedColor = NotedColor("89ddff");

#[allow(dead_code)]
pub const COLOR_GRAY: NotedColor = NotedColor("676E95");

#[allow(dead_code)]
pub const COLOR_WHITE: NotedColor = NotedColor("eeffff");

#[allow(dead_code)]
pub const COLOR_ERROR: NotedColor = NotedColor("ff5370");

#[allow(dead_code)]
pub const COLOR_COMMENT: NotedColor = NotedColor("676E95");

#[allow(dead_code)]
pub const COLOR_VARIABLE: NotedColor = NotedColor("eeffff");

#[allow(dead_code)]
pub const COLOR_LINK: NotedColor = NotedColor("80cbc4");

#[allow(dead_code)]
pub const COLOR_FUNCTION: NotedColor = NotedColor("82aaff");

#[allow(dead_code)]
pub const COLOR_KEYWORD: NotedColor = NotedColor("c792ea");

#[allow(dead_code)]
pub const COLOR_TAG: NotedColor = NotedColor("f07178");

#[allow(dead_code)]
pub const COLOR_STRING: NotedColor = NotedColor("c3e88d");

#[allow(dead_code)]
pub const COLOR_OPERATOR: NotedColor = NotedColor("89ddff");

#[allow(dead_code)]
pub const COLOR_ATTRIBUTE: NotedColor = NotedColor("ffcb6b");

#[allow(dead_code)]
pub const COLOR_NUMBER: NotedColor = NotedColor("f78c6c");

#[allow(dead_code)]
pub const COLOR_PARAMETER: NotedColor = NotedColor("f78c6c");

impl Into<Color> for NotedColor {
    fn into(self) -> Color {
        Color::hex(self.0).unwrap()
    }
}

impl Into<BackgroundColor> for NotedColor {
    fn into(self) -> BackgroundColor {
        BackgroundColor(self.into())
    }
}