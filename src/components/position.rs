use bevy::prelude::*;

use crate::constants::{
    GRID_HEIGHT, GRID_WIDTH, TILE_HEIGHT, TILE_MARGIN_HORIZONTAL, TILE_MARGIN_VERTICAL, TILE_WIDTH,
};

#[derive(Component, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        return Self { x, y };
    }
    pub fn to_transform(&self, z: f32) -> Transform {
        let x = (-(GRID_WIDTH as f32) / 2.0 + self.x as f32 + 0.5)
            * (TILE_WIDTH + TILE_MARGIN_HORIZONTAL);
        let y = (-(GRID_HEIGHT as f32) / 2.0 + self.y as f32 + 0.5)
            * (TILE_HEIGHT + TILE_MARGIN_VERTICAL);
        return Transform::from_xyz(x, y, z);
    }
    pub fn shift(&mut self, (x, y): (isize, isize)) {
        self.x = (self.x as isize + x) as usize;
        self.y = (self.y as isize + y) as usize;
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        return Position::new(value.0, value.1);
    }
}

impl From<Position> for Transform {
    fn from(pos: Position) -> Self {
        return pos.to_transform(10.0);
    }
}
