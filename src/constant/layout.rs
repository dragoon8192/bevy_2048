use bevy::{math::f32, prelude::Vec2};

#[derive(Default)]
pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub border: Option<f32>,
    pub margin: Option<Margin>,
}

#[derive(Default)]
pub struct Margin {
    pub horizontal: f32,
    pub vertical: f32,
}

impl Rect {
    pub const fn new(width: f32, height: f32) -> Self {
        return Self {
            width,
            height,
            margin: None,
            border: None,
        };
    }
    pub const fn with_margin(self, horizontal: f32, vertical: f32) -> Self {
        return Self {
            margin: Some(Margin {
                horizontal,
                vertical,
            }),
            ..self
        };
    }
    pub const fn with_border(self, border: f32) -> Self {
        return Self {
            border: Some(border),
            ..self
        };
    }
    pub fn size_2d(self) -> Vec2 {
        return Vec2::new(self.width, self.height);
    }
}

const WINDOW_WIDTH: f32 = 300.0;

pub const WINDOW: Rect = Rect::new(
    WINDOW_WIDTH,
    BODY.height + HEADER.height + HEADER_TO_BODY_MARGIN,
);
pub const HEADER: Rect = Rect::new(WINDOW_WIDTH, 80.0);
pub const HEADER_TO_BODY_MARGIN: f32 = 10.0;
pub const BODY: Rect = Rect::new(WINDOW_WIDTH, 300.0);
