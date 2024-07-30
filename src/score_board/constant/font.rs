use crate::constants::font;

pub const NAME: &str = font::MAIN_FONT_NAME;

pub mod size {
    pub const TITLE: f32 = 60.0;
    pub const MENU: f32 = 40.0;
}

pub mod color {
    use bevy::render::color::Color;
    pub const TITLE: Color = Color::GRAY;
    pub const MENU: Color = Color::GRAY;
}
