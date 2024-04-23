use bevy::ecs::system::QueryLens;
use bevy::prelude::*;
use bevy::text::{Text, Text2dBounds};

use crate::components::position::Position;
use crate::components::tile::Tile;
use crate::constants::TILE_FONT_SIZE;
use crate::constants::TILE_SIZE_2D;
use crate::structs::grid_array::GridArray;

// 盤面の状態の取得
pub fn get_tiles_layout(lens: &mut QueryLens<&Position>) -> GridArray<bool> {
    let mut tiles_layout: GridArray<bool> = GridArray::new(false);
    for pos in lens.query().iter() {
        tiles_layout.0[pos.x][pos.y] = true;
    }
    return tiles_layout;
}

// 任意の Position への Tile の追加
pub fn create_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    tile: Tile,
    position: Position,
) {
    let font = asset_server.load("fonts/Kenney Space.ttf");
    let text = Text::from_section(
        tile.to_string(),
        TextStyle {
            font: font.clone(),
            font_size: TILE_FONT_SIZE,
            color: Color::GRAY,
        },
    );
    commands
        .spawn_empty()
        .insert(tile)
        .insert(position.clone())
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::from(tile),
                custom_size: TILE_SIZE_2D,
                ..default()
            },
            transform: position.into(),
            ..default()
        })
        .insert(Text2dBundle {
            text,
            transform: position.to_transform(20.0),
            text_2d_bounds: Text2dBounds {
                size: Vec2 {
                    x: TILE_FONT_SIZE,
                    y: TILE_FONT_SIZE,
                },
            },
            ..default()
        });
}
