use bevy::ecs::system::{Query, QueryLens};
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use itertools::iproduct;
use rand_core::RngCore;
use std::collections::BTreeSet;

use super::position::get_positions_complement_set;
use crate::constants::SIDE_LENGTH;
use crate::constants::TILE_SIZE_2D;
use crate::events::move_event::MoveEvent;
use crate::states::game_state::GameState;

pub use crate::components::position::Position;
pub use crate::components::tile::Tile;

// 盤面の状態の取得
pub fn get_tiles_layout(lens: &mut QueryLens<&Position>) -> [[bool; SIDE_LENGTH]; SIDE_LENGTH] {
    let mut tiles_layout: [[bool; SIDE_LENGTH]; SIDE_LENGTH] = [[false; SIDE_LENGTH]; SIDE_LENGTH];
    for pos in lens.query().iter() {
        tiles_layout[pos.x][pos.y] = true;
    }
    return tiles_layout;
}

// 任意の Position への Tile の追加
pub fn create_tile(commands: &mut Commands, tile: Tile, position: Position) {
    commands
        .spawn_empty()
        .insert(tile)
        .insert(position.clone())
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::from(tile),
                custom_size: TILE_SIZE_2D,
                ..Default::default()
            },
            transform: position.into(),
            ..Default::default()
        });
}

// 空いた Position への Tile の追加
pub fn create_random_tile(
    mut commands: Commands,
    mut query: Query<&Position, With<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let mut lens: QueryLens<&Position> = query.transmute_lens::<&Position>();
    let candidates_of_positions: BTreeSet<Position> = get_positions_complement_set(lens);
    let rnd_n = rng.next_u64() as usize % candidates_of_positions.len();
    let position = candidates_of_positions
        .iter()
        .nth(rnd_n)
        .expect("candidates_of_positions: out of range!!")
        .clone();
    let tile = Tile(2);
    create_tile(&mut commands, tile, position);
}

// MoveEvent に基づいて Tile を移動
pub fn handle_tile_movement(
    mut move_evr: EventReader<MoveEvent>,
    mut query: Query<(&mut Transform, &mut Position), With<Tile>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut moved = false;
    for ev in move_evr.read() {
        // 移動方向と回転回数
        let (dx, dy, rot) = match ev {
            MoveEvent::Left => (-1, 0, 3),
            MoveEvent::Right => (1, 0, 1),
            MoveEvent::Up => (0, 1, 2),
            MoveEvent::Down => (0, -1, 0),
        };
        // 盤面の状態の取得
        let tiles_layout = get_tiles_layout(&mut query.transmute_lens::<&Position>());
        let mut tiles_num = tiles_layout.map(|row| row.map(|b| if b { 1 } else { 0 }));
        // 回転
        for _ in 0..rot {
            rotate_ccw(&mut tiles_num);
        }
        // 移動回数のカウント
        for i in 0..SIDE_LENGTH {
            let mut v = 0;
            for j in 0..SIDE_LENGTH {
                v += 1 - tiles_num[i][j];
                tiles_num[i][j] += v - 1;
            }
        }
        // 逆回転
        for _ in rot..4 {
            rotate_ccw(&mut tiles_num);
        }
        // 移動
        for (mut trans, mut pos) in query.iter_mut() {
            let mv = tiles_num[pos.x][pos.y];
            pos.x = (pos.x as isize + dx * mv) as usize;
            pos.y = (pos.y as isize + dy * mv) as usize;
            *trans = pos.clone().into();
        }
        moved = true;
    }
    if moved {
        next_state.set(GameState::Spawn);
    }
}

fn rotate_ccw(a: &mut [[isize; SIDE_LENGTH]; SIDE_LENGTH]) {
    let mut b: [[isize; SIDE_LENGTH]; SIDE_LENGTH] = [[0; SIDE_LENGTH]; SIDE_LENGTH];
    for (i, j) in iproduct!(0..SIDE_LENGTH, 0..SIDE_LENGTH) {
        b[i][j] = a[SIDE_LENGTH - j - 1][i];
    }
    *a = b;
}
