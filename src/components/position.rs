use bevy::prelude::*;

use crate::constants::SIDE_LENGTH;
use crate::constants::TILE_SIZE;

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
        let x = (-(SIDE_LENGTH as f32) / 2.0 + self.x as f32 + 0.5) * (TILE_SIZE * 1.05);
        let y = (-(SIDE_LENGTH as f32) / 2.0 + self.y as f32 + 0.5) * (TILE_SIZE * 1.05);
        return Transform::from_xyz(x, y, z);
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