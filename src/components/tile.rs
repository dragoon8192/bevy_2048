use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Tile(u64);

impl Tile {
    fn double() {
        self.0 *= 2;
    }
}

impl From<Tile> for Color {
    fn from(Tile(num): Tile) -> Self {
        match num {
            2 => Color::GOLD,
            4 => Color::ORANGE,
            8 => Color::ORANGE_RED,
            _ => Color::RED,
            // TODO
        }
    }
}
