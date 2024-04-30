use bevy::prelude::*;

use crate::constants::{TILE_COLOR_0, TILE_COLOR_1};

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Tile(pub u8);

impl Tile {
    pub fn double(&mut self) {
        self.0 += 1;
    }
}

impl From<Tile> for Color {
    fn from(Tile(rank): Tile) -> Self {
        // rank = 1 -> r = 0.0, rank >= 11 -> r = 1.0
        let r: f32 = (rank - 1) as f32 / 10.0;
        return TILE_COLOR_0 * (1.0 - r) + TILE_COLOR_1 * r;
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        return 2_usize.pow(self.0 as u32).to_string();
    }
}
