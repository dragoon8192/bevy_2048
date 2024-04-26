use bevy::prelude::*;

use crate::constants::{TILE_COLOR_0, TILE_COLOR_1};

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Tile(pub u64);

impl Tile {
    pub fn double(&mut self) {
        self.0 *= 2;
    }
}

impl From<Tile> for Color {
    fn from(Tile(num): Tile) -> Self {
        // num = 2 -> r = 0.0, num >= 2048 -> r = 1.0
        let r: f32 = (u64::BITS - num.leading_zeros() - 2).min(10) as f32 / 10.0;
        return TILE_COLOR_0 * (1.0 - r) + TILE_COLOR_1 * r;
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        return self.0.to_string();
    }
}
