use crate::constant::color::{self, Color};

pub const MAIN: Color = color::board::MAIN;
pub const BORDER: Color = color::board::SUB;

pub mod text {
    use crate::constant::color::{self, Color};
    pub const HEADER: Color = color::text::MAIN;
    pub const BODY: Color = color::text::MAIN;
}
