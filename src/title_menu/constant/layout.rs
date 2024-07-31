pub mod header {
    use crate::constant::layout;
    pub const WIDTH: f32 = layout::MAIN_BOARD_WIDTH;
    pub const HEIGHT: f32 = layout::SCORE_BOARD_HEIGHT;
}

pub mod body {
    use crate::constant::layout;
    pub const WIDTH: f32 = layout::MAIN_BOARD_WIDTH;
    pub const HEIGHT: f32 = layout::MAIN_BOARD_HEIGHT;
}

pub mod button {
    pub const WIDTH: f32 = 250.0;
    pub const HEIGHT: f32 = 72.0;
    pub const BORDER: f32 = 8.0;
}
