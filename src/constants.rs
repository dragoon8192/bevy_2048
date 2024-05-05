use bevy::{math::f32, prelude::Vec2, render::color::Color};

pub const WINDOW_WIDTH: f32 = MAIN_BOARD_WIDTH;
pub const WINDOW_HEIGHT: f32 = MAIN_BOARD_HEIGHT + SCORE_BOARD_HEIGHT + MAIN_AND_SCORE_BOARD_MARGIN;

pub const GRID_WIDTH: usize = 4;
pub const GRID_HEIGHT: usize = 4;

pub const TILE_WIDTH: f32 = 60.0;
pub const TILE_HEIGHT: f32 = 60.0;
pub const TILE_SIZE_2D: Option<Vec2> = Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT));
pub const TILE_MARGIN_HORIZONTAL: f32 = 8.0;
pub const TILE_MARGIN_VERTICAL: f32 = 8.0;
pub const TILE_FONT_SIZE: f32 = 60.0;

pub const MAIN_BOARD_WIDTH: f32 = TILE_WIDTH * GRID_WIDTH as f32
    + TILE_MARGIN_HORIZONTAL * (GRID_WIDTH - 1) as f32
    + MAIN_BOARD_PADDING * 2.0;
pub const MAIN_BOARD_HEIGHT: f32 = TILE_HEIGHT * GRID_HEIGHT as f32
    + TILE_MARGIN_VERTICAL * (GRID_HEIGHT - 1) as f32
    + MAIN_BOARD_PADDING * 2.0;
pub const MAIN_BOARD_SIZE_2D: Option<Vec2> = Some(Vec2::new(MAIN_BOARD_WIDTH, MAIN_BOARD_HEIGHT));
pub const MAIN_BOARD_PADDING: f32 = 18.0;

pub const SCORE_BOARD_HEIGHT: f32 = 80.0;
pub const SCORE_BOARD_SIZE_2D: Option<Vec2> = Some(Vec2::new(MAIN_BOARD_WIDTH, SCORE_BOARD_HEIGHT));
pub const MAIN_AND_SCORE_BOARD_MARGIN: f32 = 10.0;
pub const SCORE_FONT_SIZE: f32 = 40.0;

pub const TILE_COLOR_0: Color = Color::rgb(0.0, 0.922, 0.5);
pub const TILE_COLOR_1: Color = Color::rgb(0.0, 0.5, 0.922);
pub const TILE_TEXT_COLOR: Color = Color::GRAY;

pub const BOARD_COLOR_0: Color = Color::BEIGE;
pub const BOARD_COLOR_1: Color = Color::GRAY;

pub const SCORE_TEXT_COLOR: Color = Color::GRAY;
