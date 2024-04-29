use bevy::{math::f32, prelude::Vec2, render::color::Color};

pub const WINDOW_SIZE: f32 = 500.0;

pub const GRID_WIDTH: usize = 4;
pub const GRID_HEIGHT: usize = 4;

pub const TILE_WIDTH: f32 = 60.0;
pub const TILE_HEIGHT: f32 = 60.0;
pub const TILE_SIZE_2D: Option<Vec2> = Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT));
pub const TILE_MARGIN_HORIZONTAL: f32 = 5.0;
pub const TILE_MARGIN_VERTICAL: f32 = 5.0;
pub const TILE_FONT_SIZE: f32 = 60.0;

pub const BOARD_SIZE_2D: Option<Vec2> = Some(Vec2::new(
    TILE_WIDTH * GRID_WIDTH as f32
        + TILE_MARGIN_HORIZONTAL * (GRID_WIDTH - 1) as f32
        + BOARD_PADDING * 2.0,
    TILE_HEIGHT * GRID_HEIGHT as f32
        + TILE_MARGIN_VERTICAL * (GRID_HEIGHT - 1) as f32
        + BOARD_PADDING * 2.0,
));
pub const BOARD_PADDING: f32 = 22.5;

pub const TILE_COLOR_0: Color = Color::rgb(0.0, 0.922, 0.5);
pub const TILE_COLOR_1: Color = Color::rgb(0.0, 0.5, 0.922);

pub const BOARD_COLOR_0: Color = Color::BEIGE;
pub const BOARD_COLOR_1: Color = Color::GRAY;
