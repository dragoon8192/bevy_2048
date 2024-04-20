use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Tile(u64);

impl Tile {
    pub fn double(&mut self) {
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

impl ToString for Tile {
    fn to_string(&self) -> String {
        return self.0.to_string();
    }
}
