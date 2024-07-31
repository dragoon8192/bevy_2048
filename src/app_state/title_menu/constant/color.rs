use crate::constant::color::{self, Color};

pub const MAIN: Color = color::background::MAIN;
pub const BORDER: Color = color::background::SUB;

pub mod text {
    use crate::constant::color::{self, Color};
    pub const HEADER: Color = color::text::MAIN;
    pub const BODY: Color = color::text::MAIN;
}
