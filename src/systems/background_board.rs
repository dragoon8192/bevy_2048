use crate::components::position::Position;
use crate::constants::BOARD_SIZE_2D;
use crate::constants::SIDE_LENGTH;
use crate::constants::TILE_SIZE_2D;
use bevy::prelude::*;
use itertools::iproduct;

pub use crate::components::background_board::Background;

pub fn create_background_board(commands: &mut Commands) {
    // 大きな盤
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: BOARD_SIZE_2D,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Background);
    for (i, j) in iproduct!(0..SIDE_LENGTH, 0..SIDE_LENGTH) {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: TILE_SIZE_2D,
                    ..Default::default()
                },
                transform: Position::new(i, j).to_transform(0.0),
                ..Default::default()
            })
            .insert(Background);
    }
}
