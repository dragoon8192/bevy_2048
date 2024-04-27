use crate::{
    components::{background_board::Background, position::Position},
    constants::{
        BOARD_COLOR_0, BOARD_COLOR_1, BOARD_SIZE_2D, GRID_HEIGHT, GRID_WIDTH, TILE_SIZE_2D,
    },
};
use bevy::prelude::*;
use itertools::iproduct;

#[derive(Bundle)]
struct BackgroundBoardBundle {
    background: Background,
    sprite_bunble: SpriteBundle,
}

impl Default for BackgroundBoardBundle {
    fn default() -> Self {
        return Self {
            background: Background,
            sprite_bunble: SpriteBundle {
                sprite: Sprite {
                    color: BOARD_COLOR_0,
                    custom_size: BOARD_SIZE_2D,
                    ..default()
                },
                ..default()
            },
        };
    }
}

impl BackgroundBoardBundle {
    fn child_builder(parent: &mut ChildBuilder) {
        for (i, j) in iproduct!(0..GRID_WIDTH, 0..GRID_HEIGHT) {
            parent.spawn(BackgroundTileBundle::new(i, j));
        }
    }
}

#[derive(Bundle)]
struct BackgroundTileBundle {
    background: Background,
    sprite_bunble: SpriteBundle,
}

impl Default for BackgroundTileBundle {
    fn default() -> Self {
        return Self {
            background: Background,
            sprite_bunble: SpriteBundle {
                sprite: Sprite {
                    color: BOARD_COLOR_1,
                    custom_size: TILE_SIZE_2D,
                    ..default()
                },
                ..default()
            },
        };
    }
}

impl BackgroundTileBundle {
    fn new(i: usize, j: usize) -> Self {
        let mut val = Self::default();
        val.sprite_bunble.transform = Position::new(i, j).to_transform(0.0);
        return val;
    }
}

pub fn create_background_board(commands: &mut Commands) {
    // 大きな盤
    commands
        .spawn(BackgroundBoardBundle::default())
        .with_children(BackgroundBoardBundle::child_builder);
}
