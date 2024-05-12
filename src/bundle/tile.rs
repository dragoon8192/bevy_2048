use bevy::prelude::*;
use bevy::text::{Text, Text2dBounds};

use crate::components::main_board::MainBoard;
use crate::{
    components::{position::Position, tile::Tile},
    constants::{TILE_FONT_SIZE, TILE_SIZE_2D, TILE_TEXT_COLOR},
};

// // 盤面の状態の取得
// pub fn get_tiles_layout(lens: &mut QueryLens<&Position>) -> GridArray<bool> {
//     let mut tiles_layout: GridArray<bool> = GridArray::new(false);
//     for pos in lens.query().iter() {
//         tiles_layout.0[pos.x][pos.y] = true;
//     }
//     return tiles_layout;
// }

#[derive(Bundle, Clone)]
struct TileBundle {
    tile: Tile,
    position: Position,
    sprite_bunble: SpriteBundle,
}

impl Default for TileBundle {
    fn default() -> Self {
        let tile = Tile(1);
        let position = Position { x: 0, y: 0 };
        return Self {
            tile,
            position,
            sprite_bunble: SpriteBundle {
                sprite: Sprite {
                    color: Color::from(tile),
                    custom_size: Some(TILE_SIZE_2D),
                    ..default()
                },
                transform: position.into(),
                ..default()
            },
        };
    }
}

impl TileBundle {
    fn new(tile: Tile, position: Position) -> Self {
        let mut val = Self::default();
        val.tile = tile;
        val.position = position;
        val.sprite_bunble.transform = position.into();
        return val;
    }
    fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        let text = Text::from_section(
            self.tile.to_string(),
            TextStyle {
                font,
                font_size: TILE_FONT_SIZE,
                color: TILE_TEXT_COLOR,
            },
        );
        return move |parent| {
            parent.spawn(Text2dBundle {
                text,
                transform: Transform::from_xyz(0.0, 0.0, 5.0),
                text_2d_bounds: Text2dBounds {
                    size: Vec2 {
                        x: TILE_FONT_SIZE,
                        y: TILE_FONT_SIZE,
                    },
                },
                ..default()
            });
        };
    }
}

#[derive(Event, Eq, PartialEq)]
pub struct TileSpawnEvent {
    pub tile: Tile,
    pub position: Position,
}

// 任意の Position への Tile の追加
pub fn spawn_tiles(
    mut tile_spawn_evr: EventReader<TileSpawnEvent>,
    mut commands: Commands,
    query_p: Query<Entity, With<MainBoard>>,
    asset_server: Res<AssetServer>,
) {
    for ev in tile_spawn_evr.read() {
        let font = asset_server.load("fonts/Kenney Space.ttf");
        let tile_bundle = TileBundle::new(ev.tile, ev.position);
        let child = commands
            .spawn(tile_bundle.clone())
            .with_children(tile_bundle.child_builder(font))
            .id();
        let parent = query_p.single();
        commands.entity(parent).push_children(&[child]);
    }
}
