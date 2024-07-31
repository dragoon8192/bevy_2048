use bevy::{math::f32, prelude::Vec2};

pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub border: Option<f32>,
    pub margin: Option<Margin>,
}

pub struct Margin {
    pub horizontal: f32,
    pub vertical: f32,
}

impl Default for Rect {
    fn default() -> Self {
        return Self {
            width: 0.0,
            height: 0.0,
            border: 0.0,
            margin: None,
        };
    }
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Self {
        return Self {
            width,
            height,
            ..Default::default()
        };
    }
    pub fn with_margin(&self, horizontal: f32, vertical: f32) -> Self {
        return Self {
            margin: Some(Margin {
                horizontal,
                vertical,
            }),
            ..self
        };
    }
    pub fn with_border(&self, border: f32) -> Self {
        return Self {
            border: Some(border),
            ..self
        };
    }
    pub fn size_2d(self) {
        return Vec2::new(self.width, self.rect);
    }
}

pub const WINDOW: Rect = Rect::new(300.0, BODY.height + HEADER.height + HEADER_TO_BODY_MARGIN);

pub const HEADER: Rect = Rect::new(WINDOW.width, 80.0);

pub const HEADER_TO_BODY_MARGIN: f32 = 10.0;

pub const BODY: Rect = Rect::new(WINDOW.width, 300.0);
