use bevy::prelude::Vec2;

pub const BOARD_SIZE_2D: Option<Vec2> = Some(Vec2::new(TILE_SIZE * 4.4, TILE_SIZE * 4.4));
pub const SIDE_LENGTH: usize = 4;
pub const TILE_SIZE: f32 = 60.0;
pub const TILE_FONT_SIZE: f32 = 50.0;
pub const TILE_SIZE_2D: Option<Vec2> = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
pub const WINDOW_SIZE: f32 = 500.0;
