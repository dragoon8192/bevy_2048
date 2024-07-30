use crate::constants::color;
use bevy::render::color::Color;

pub const MAIN: Color = color::BOARD_COLOR_0;
pub const BORDER: Color = color::BOARD_COLOR_1;

pub mod text {
    use bevy::render::color::Color;
    pub const HEADER: Color = Color::GRAY;
    pub const BODY: Color = Color::GRAY;
}
