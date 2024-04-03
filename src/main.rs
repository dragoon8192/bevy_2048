use bevy::app::*;
use bevy::ecs::system::{Query, QueryLens};
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;
use itertools::iproduct;
use rand_core::RngCore;
use std::collections::BTreeSet;

const WINDOW_SIZE: f32 = 500.0;
const TILE_SIZE: f32 = 60.0;
const TILE_SIZE_2D: Option<Vec2> = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
const BOARD_SIZE_2D: Option<Vec2> = Some(Vec2::new(TILE_SIZE * 4.4, TILE_SIZE * 4.4));
const SIDE_LENGTH: usize = 4;

#[derive(Component, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        return Self { x, y };
    }
    fn to_transform(&self, z: f32) -> Transform {
        let x = (-(SIDE_LENGTH as f32) / 2.0 + self.x as f32 + 0.5) * (TILE_SIZE * 1.05);
        let y = (-(SIDE_LENGTH as f32) / 2.0 + self.y as f32 + 0.5) * (TILE_SIZE * 1.05);
        return Transform::from_xyz(x, y, z);
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        return Position::new(value.0, value.1);
    }
}

impl From<Position> for Transform {
    fn from(pos: Position) -> Self {
        return pos.to_transform(10.0);
    }
}

#[derive(Component, Clone, Copy)]
struct Tile(u64);

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

fn create_tile(commands: &mut Commands, tile: Tile, position: Position) {
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

fn move_tiles_system(
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

        for _ in 0..rot {
            rotate_ccw(&mut tiles_num);
        }

        for i in 0..SIDE_LENGTH {
            let mut v = 0;
            for j in 0..SIDE_LENGTH {
                v += 1 - tiles_num[i][j];
                tiles_num[i][j] += v - 1;
            }
        }

        for _ in rot..4 {
            rotate_ccw(&mut tiles_num);
        }

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

#[derive(Component)]
struct Background;

fn create_background_board(commands: &mut Commands) {
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    create_background_board(&mut commands);

    for (i, j, num) in [(1, 0, 2), (3, 0, 4), (1, 3, 8)] {
        create_tile(&mut commands, Tile(num), Position::new(i, j));
    }
}

#[derive(Event, PartialEq, Eq)]
enum MoveEvent {
    Left,
    Right,
    Up,
    Down,
}

fn send_move_event_from_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut move_evw: EventWriter<MoveEvent>,
) {
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        move_evw.send(MoveEvent::Left);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        move_evw.send(MoveEvent::Right);
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        move_evw.send(MoveEvent::Up);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        move_evw.send(MoveEvent::Down);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Move,
    Spawn,
}

fn return_to_move_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Move);
}

// 盤面の状態の取得
fn get_tiles_layout(lens: &mut QueryLens<&Position>) -> [[bool; SIDE_LENGTH]; SIDE_LENGTH] {
    let mut tiles_layout: [[bool; SIDE_LENGTH]; SIDE_LENGTH] = [[false; SIDE_LENGTH]; SIDE_LENGTH];
    for pos in lens.query().iter() {
        tiles_layout[pos.x][pos.y] = true;
    }
    return tiles_layout;
}

fn get_positions_set(lens: &mut QueryLens<&Position>) -> BTreeSet<Position> {
    let query: Query<'_, '_, &Position> = lens.query();
    let iter = query.iter().cloned();
    return BTreeSet::from_iter(iter);
}

fn positions_univ_set() -> BTreeSet<Position> {
    return BTreeSet::from_iter(iproduct!(0..SIDE_LENGTH, 0..SIDE_LENGTH).map(Position::from));
}

fn get_positions_complement_set(lens: &mut QueryLens<&Position>) -> BTreeSet<Position> {
    let query: Query<'_, '_, &Position> = lens.query();
    let iter = query.iter().cloned();
    let set: BTreeSet<Position> = BTreeSet::from_iter(iter);
    return positions_univ_set().difference(&set).cloned().collect();
}

fn create_random_tile(
    mut commands: Commands,
    mut query: Query<&Position, With<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let mut lens: QueryLens<&Position> = query.transmute_lens::<&Position>();
    let candidates_of_positions: BTreeSet<Position> = get_positions_complement_set(&mut lens);
    let rnd_n = rng.next_u64() as usize % candidates_of_positions.len();
    let position = candidates_of_positions
        .iter()
        .nth(rnd_n)
        .expect("candidates_of_positions: out of range!!")
        .clone();
    let tile = Tile(2);
    create_tile(&mut commands, tile, position);
}

fn main() {
    let window = Window {
        title: "2048".to_string(),
        resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
        ..default()
    };
    let primary_window = Some(window);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window,
            ..default()
        }))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_systems(Startup, setup)
        .init_state::<GameState>()
        .add_event::<MoveEvent>()
        .add_systems(
            Update,
            (move_tiles_system, send_move_event_from_keyboard).run_if(in_state(GameState::Move)),
        )
        .add_systems(
            Update,
            (create_random_tile, return_to_move_state)
                .chain()
                .run_if(in_state(GameState::Spawn)),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
