use crate::constant::font;

pub const NAME: &str = font::NAME;

pub mod size {
    use super::font::size;
    pub const HEADER: f32 = size::MAIN;
    pub const BODY: f32 = size::SUB;
}
