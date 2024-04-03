use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Tile(pub u64);

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
