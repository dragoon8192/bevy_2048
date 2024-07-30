use crate::constants::color::{self, Color};

pub const MAIN: Color = color::board::MAIN;
pub const BORDER: Color = color::board::SUB;

pub mod text {
    use crate::constants::color::{self, Color};
    pub const HEADER: Color = color::board::TEXT;
    pub const BODY: Color = color::board::TEXT;
}
