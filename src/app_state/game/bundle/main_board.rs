use super::super::{
    component::{main_board::MainBoard, position::Position},
    constant::{color, layout},
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
                    color: color::background::MAIN,
                    custom_size: Some(layout::BODY.size_2d()),
                    ..default()
                },
                transform: Transform::from_xyz(
                    0.0,
                    -(layout::HEADER.height + layout::HEADER_TO_BODY_MARGIN) / 2.0,
                    0.0,
                ),
                ..default()
            },
        };
    }
}

impl MainBoardBundle {
    fn child_builder(parent: &mut ChildBuilder) {
        for (i, j) in iproduct!(0..layout::GRID_WIDTH, 0..layout::GRID_WIDTH) {
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
                    color: color::background::SUB,
                    custom_size: Some(layout::TILE.size_2d()),
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
