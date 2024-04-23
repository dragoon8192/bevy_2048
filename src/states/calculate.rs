use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::QueryLens;
use bevy::prelude::*;
use std::collections::VecDeque;
use std::result::Result;

use crate::states::game_state::GameState;
use crate::states::input::PlayerInputEvent;

use crate::components::position::Position;
use crate::components::tile::Tile;
use crate::error::handle_query_entity_errors;
use crate::structs::grid_array::{GridArray, RotatedGridArray};
use crate::structs::quater_turn::QuarterTurn;

pub struct CalculatePlugin;

impl Plugin for CalculatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SlicedMovementEvent>()
            .add_event::<TileMovementEvent>()
            .add_systems(
                OnEnter(GameState::Calculate),
                (
                    handle_player_input,
                    calc_sliced_movement.pipe(handle_query_entity_errors),
                )
                    .chain(),
            )
    }
}

#[derive(Event)]
pub struct SlicedMovementEvent(Vec<Option<Entity>>, QuarterTurn);

#[derive(Event)]
pub enum TileMovementEvent {
    OneStep(Entity, QuarterTurn),
    Merge(Entity, Entity, QuarterTurn),
}

// 盤面の状態を Entity と紐づけて取得
pub fn get_tiles_layout_with_entity(
    lens: &mut QueryLens<(Entity, &Position)>,
) -> GridArray<Option<Entity>> {
    let mut tiles_layout: GridArray<Option<Entity>> = GridArray::new(None);
    for (e, pos) in lens.query().iter() {
        tiles_layout.0[pos.x][pos.y] = Some(e);
    }
    return tiles_layout;
}

// PlayerInputEvent に基づいて SlicedMovementEvent を発行
pub fn handle_player_input(
    mut input_evr: EventReader<PlayerInputEvent>,
    mut sliced_move_evw: EventWriter<SlicedMovementEvent>,
    mut query: Query<(Entity, &mut Transform, &mut Position), With<Tile>>,
) {
    dbg!("System: handle_player_input");
    for ev in input_evr.read() {
        // 移動方向と回転回数
        // tiles_layout[x][y] にアクセスするので、行列の並び方と 90deg ずれることに注意
        // 回転させなければ下に落ちる
        // ex: 反時計回りに 90deg 回転させて考えることで左に落ちる
        let turn: QuarterTurn = match ev {
            PlayerInputEvent::Down => QuarterTurn::Deg000,
            PlayerInputEvent::Left => QuarterTurn::Deg090,
            PlayerInputEvent::Up => QuarterTurn::Deg180,
            PlayerInputEvent::Right => QuarterTurn::Deg270,
        };
        // 盤面の状態の取得
        let tiles_layout: RotatedGridArray<Option<Entity>> = RotatedGridArray {
            grid_array: get_tiles_layout_with_entity(
                &mut query.transmute_lens::<(Entity, &Position)>(),
            ),
            turn,
        };
        dbg!(&tiles_layout);
        let vec: Vec<Vec<Option<Entity>>> = tiles_layout.into();
        // 動いた方向にスライスしてそれぞれについて SlicedMovementEvent を発行
        for down_axis in vec.into_iter() {
            sliced_move_evw.send(SlicedMovementEvent(down_axis, turn));
        }
    }
}

fn shift_tiles_one_step(
    tile_entitys: Vec<Option<Entity>>,
    turn: QuarterTurn,
    tile_move_evw: &mut EventWriter<TileMovementEvent>,
) {
    for option in tile_entitys {
        if let Some(e) = option {
            tile_move_evw.send(TileMovementEvent::OneStep(e, turn));
        }
    }
}

fn calc_tiles_slice(
    tile_entitys: &mut VecDeque<Option<Entity>>,
    turn: QuarterTurn,
    tile_move_evw: &mut EventWriter<TileMovementEvent>,
    query: &Query<&Tile>,
) -> Result<(), QueryEntityError> {
    match tile_entitys.pop_front() {
        // tile_entitys is empty. i.e. tile_entitys = [].
        None => {
            return Ok(());
        }
        // 要素を持つが、最初が空白だった場合. i.e. tile_entitys = None: _.
        Some(None) => {
            shift_tiles_one_step((*tile_entitys).clone().into(), turn, tile_move_evw);
            return calc_tiles_slice(tile_entitys, turn, tile_move_evw, &query);
        }
        // 最初の要素が Tile だった場合. i.e. tile_entitys = Some(e0): _.
        Some(Some(e0)) => {
            match tile_entitys.pop_front() {
                // 1つしか要素を持たなかった場合. i.e. tile_entitys = Some(e0): [].
                None => {
                    return Ok(());
                }
                // tile_entitys = Some(e0): None: _.
                Some(None) => {
                    shift_tiles_one_step((*tile_entitys).clone().into(), turn, tile_move_evw);
                    tile_entitys.push_front(Some(e0));
                    return calc_tiles_slice(tile_entitys, turn, tile_move_evw, &query);
                }
                // tile_entitys = Some(e0): Some(e1): _.
                Some(Some(e1)) => {
                    let tile0 = query.get(e0)?;
                    let tile1 = query.get(e1)?;
                    if tile0 == tile1 {
                        tile_move_evw.send(TileMovementEvent::Merge(e0, e1, turn));
                        shift_tiles_one_step((*tile_entitys).clone().into(), turn, tile_move_evw);
                        return calc_tiles_slice(tile_entitys, turn, tile_move_evw, &query);
                    } else {
                        tile_entitys.push_front(Some(e1));
                        return calc_tiles_slice(tile_entitys, turn, tile_move_evw, &query);
                    }
                }
            }
        }
    }
}

pub fn calc_sliced_movement(
    mut sliced_move_evr: EventReader<SlicedMovementEvent>,
    mut tile_move_evw: EventWriter<TileMovementEvent>,
    query: Query<&Tile>,
    mut next_state: ResMut<NextState<GameState>>,
) -> Result<(), QueryEntityError> {
    dbg!("System: calc_sliced_movement");
    for SlicedMovementEvent(tile_entitys, turn) in sliced_move_evr.read() {
        let mut tile_entitys: VecDeque<Option<Entity>> = (*tile_entitys).clone().into();

        calc_tiles_slice(&mut tile_entitys, *turn, &mut tile_move_evw, &query)?;
    }
    next_state.set(GameState::Move);
    dbg!(GameState::Move);
    return Ok(());
}
