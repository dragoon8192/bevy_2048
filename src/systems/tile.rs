use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::{Query, QueryLens};
use bevy::prelude::*;
use bevy::text::{Text, Text2dBounds};
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use rand_core::RngCore;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::result::Result;

use super::position::get_positions_complement_set;
use crate::constants::GRID_HEIGHT;
use crate::constants::GRID_WIDTH;
use crate::constants::TILE_FONT_SIZE;
use crate::constants::TILE_SIZE_2D;
use crate::events::player_input_event::PlayerInputEvent;
use crate::states::game_state::GameState;

pub use crate::components::position::Position;
pub use crate::components::tile::Tile;

// grid : GridArray<T> は [0][0] から [GRID_WIDTH - 1][GRID_HEIGHT - 1] までの成分を持つ
struct GridArray<T>([[T; GRID_HEIGHT]; GRID_WIDTH]);

impl<T: Copy> GridArray<T> {
    pub fn new(a: T) -> Self {
        return GridArray([[a; GRID_HEIGHT]; GRID_WIDTH]);
    }
}

struct RotatedGridArray<T> {
    grid_array: GridArray<T>,
    turn: QuarterTurn,
}

impl<T> RotatedGridArray<T> {
    pub fn new(grid_array: GridArray<T>, turn: QuarterTurn) -> Self {
        Self { grid_array, turn }
    }
    pub fn width(&self) -> usize {
        match self.turn {
            QuarterTurn::Deg000 | QuarterTurn::Deg180 => return GRID_WIDTH,
            QuarterTurn::Deg090 | QuarterTurn::Deg270 => return GRID_HEIGHT,
        }
    }
    pub fn height(&self) -> usize {
        match self.turn {
            QuarterTurn::Deg000 | QuarterTurn::Deg180 => return GRID_HEIGHT,
            QuarterTurn::Deg090 | QuarterTurn::Deg270 => return GRID_WIDTH,
        }
    }
    fn get(&self, i: usize, j: usize) -> Option<&T> {
        let get2 = |i, j| {
            self.grid_array
                .0
                .get(i)
                .and_then(|y_axis: &[T; GRID_HEIGHT]| (*y_axis).get(j))
        };
        match self.turn {
            QuarterTurn::Deg000 => return get2(i, j),
            QuarterTurn::Deg090 => return get2(j, GRID_HEIGHT - 1 - i),
            QuarterTurn::Deg180 => return get2(GRID_WIDTH - 1 - i, GRID_HEIGHT - 1 - j),
            QuarterTurn::Deg270 => return get2(GRID_WIDTH - j, i),
        }
    }
}

impl<T: Clone> From<RotatedGridArray<T>> for Vec<Vec<T>> {
    fn from(grid: RotatedGridArray<T>) -> Self {
        let mut vec: Vec<Vec<T>> = vec![];
        for i in 0..grid.width() {
            for j in 0..grid.height() {
                vec[i][j] = grid.get(i, j).unwrap().clone();
            }
        }
        return vec;
    }
}

impl<T: Clone> From<RotatedGridArray<T>> for VecDeque<VecDeque<T>> {
    fn from(val: RotatedGridArray<T>) -> Self {
        let vec_vec: Vec<Vec<T>> = val.into();
        return vec_vec.into_iter().map(|vec| vec.into()).collect();
    }
}

#[derive(Default, Clone, Copy)]
enum QuarterTurn {
    #[default]
    Deg000,
    Deg090,
    Deg180,
    Deg270,
}

impl QuarterTurn {
    fn downward_unit(&self) -> (isize, isize) {
        match self {
            QuarterTurn::Deg000 => return (0, -1),
            QuarterTurn::Deg090 => return (-1, 0),
            QuarterTurn::Deg180 => return (0, 1),
            QuarterTurn::Deg270 => return (1, 0),
        }
    }
}

// 盤面の状態の取得
pub fn get_tiles_layout(lens: &mut QueryLens<&Position>) -> GridArray<bool> {
    let mut tiles_layout: GridArray<bool> = GridArray::new(false);
    for pos in lens.query().iter() {
        tiles_layout.0[pos.x][pos.y] = true;
    }
    return tiles_layout;
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
    dbg!(position);
}

// 空いた Position への Tile の追加
pub fn create_random_tile(
    mut commands: Commands,
    mut query: Query<&Position, With<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    asset_server: Res<AssetServer>,
) {
    let lens: QueryLens<&Position> = query.transmute_lens::<&Position>();
    let candidates_of_positions: BTreeSet<Position> = get_positions_complement_set(lens);
    let rnd_n = rng.next_u64() as usize % candidates_of_positions.len();
    let position = candidates_of_positions
        .iter()
        .nth(rnd_n)
        .expect("candidates_of_positions: out of range!!")
        .clone();
    let tile = Tile(2);
    create_tile(&mut commands, &asset_server, tile, position);
}

// PlayerInputEvent に基づいて SlicedMovementEvent を発行
pub fn handle_player_input(
    mut input_evr: EventReader<PlayerInputEvent>,
    mut sliced_move_evw: EventWriter<SlicedMovementEvent>,
    mut query: Query<(Entity, &mut Transform, &mut Position), With<Tile>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
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
        let vec: Vec<Vec<Option<Entity>>> = tiles_layout.into();
        // 動いた方向にスライスしてそれぞれについて SlicedMovementEvent を発行
        for down_axis in vec.into_iter() {
            sliced_move_evw.send(SlicedMovementEvent(down_axis, turn));
        }
        next_state.set(GameState::Spawn);
    }
}

#[derive(Event)]
pub struct SlicedMovementEvent(Vec<Option<Entity>>, QuarterTurn);

#[derive(Event)]
pub enum TileMovementEvent {
    OneStep(Entity, QuarterTurn),
    Merge(Entity, Entity, QuarterTurn),
}

fn shift_tiles_one_step(
    tile_entitys: Vec<Option<Entity>>,
    turn: QuarterTurn,
    tile_move_evw: &mut EventWriter<TileMovementEvent>,
) {
    tile_entitys
        .into_iter()
        .flatten()
        .map(|e| tile_move_evw.send(TileMovementEvent::OneStep(e, turn)));
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
) -> Result<(), QueryEntityError> {
    for SlicedMovementEvent(tile_entitys, turn) in sliced_move_evr.read() {
        let mut tile_entitys: VecDeque<Option<Entity>> = (*tile_entitys).clone().into();

        calc_tiles_slice(&mut tile_entitys, *turn, &mut tile_move_evw, &query);
    }
    return Ok(());
}
pub fn move_tiles(
    mut tile_move_evr: EventReader<TileMovementEvent>,
    mut query: Query<(&Position, &mut Tile)>,
    mut commands: Commands,
) -> Result<(), QueryEntityError> {
    for ev in tile_move_evr.read() {
        match ev {
            &TileMovementEvent::OneStep(e, turn) => {
                let (pos, _) = query.get(e)?;
                pos.shift(turn.downward_unit());
            }
            &TileMovementEvent::Merge(e0, e1, _) => {
                let (_, mut tile0) = query.get_mut(e0)?;
                tile0.double();
                commands.entity(e1).despawn();
            }
        }
    }
    return Ok(());
}
