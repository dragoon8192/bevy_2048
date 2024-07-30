pub use bevy::render::color::Color;

pub const BACKGROUND: Color = Color::GRAY;

pub mod text {
    use bevy::render::color::Color;
    pub const MAIN: Color = Color::GRAY;
}
pub mod board {
    use bevy::render::color::Color;
    pub const MAIN: Color = Color::BEIGE;
    pub const SUB: Color = Color::GRAY;
}

pub mod tile {
    use bevy::render::color::Color;
    pub const MAIN: Color = Color::rgb(0.0, 0.922, 0.5);
    pub const SUB: Color = Color::rgb(0.0, 0.5, 0.922);
}
