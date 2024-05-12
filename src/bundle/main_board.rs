use crate::{
    components::{main_board::MainBoard, position::Position},
    constants::{
        BOARD_COLOR_0, BOARD_COLOR_1, GRID_HEIGHT, GRID_WIDTH, MAIN_BOARD_SIZE_2D,
        SCORE_BOARD_HEIGHT, TILE_SIZE_2D,
    },
};
use bevy::prelude::*;
use itertools::iproduct;

// 背景の大きな盤
#[derive(Bundle)]
struct MainBoardBundle {
    marker: MainBoard,
    sprite_bunble: SpriteBundle,
}

impl Default for MainBoardBundle {
    fn default() -> Self {
        return Self {
            marker: MainBoard,
            sprite_bunble: SpriteBundle {
                sprite: Sprite {
                    color: BOARD_COLOR_0,
                    custom_size: Some(MAIN_BOARD_SIZE_2D),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, -SCORE_BOARD_HEIGHT / 2.0, 0.0),
                ..default()
            },
        };
    }
}

impl MainBoardBundle {
    fn child_builder(parent: &mut ChildBuilder) {
        for (i, j) in iproduct!(0..GRID_WIDTH, 0..GRID_HEIGHT) {
            parent.spawn(MainBoardTileBundle::new(i, j));
        }
    }
}

// 背景の小さなタイル
#[derive(Bundle)]
struct MainBoardTileBundle {
    sprite_bunble: SpriteBundle,
}

impl Default for MainBoardTileBundle {
    fn default() -> Self {
        return Self {
            sprite_bunble: SpriteBundle {
                sprite: Sprite {
                    color: BOARD_COLOR_1,
                    custom_size: Some(TILE_SIZE_2D),
                    ..default()
                },
                ..default()
            },
        };
    }
}

impl MainBoardTileBundle {
    fn new(i: usize, j: usize) -> Self {
        let mut val = Self::default();
        val.sprite_bunble.transform = Position::new(i, j).to_transform(5.0);
        return val;
    }
}

pub fn create_main_board(mut commands: Commands) {
    commands
        .spawn(MainBoardBundle::default())
        .with_children(MainBoardBundle::child_builder);
}
