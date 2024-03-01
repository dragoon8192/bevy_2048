use bevy::prelude::*;
use itertools::iproduct;

const WINDOW_SIZE: f32 = 500.0;
const TILE_SIZE: f32 = 60.0;
const TILE_SIZE_2D: Option<Vec2> = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
const BOARD_SIZE_2D: Option<Vec2> = Some(Vec2::new(TILE_SIZE * 4.4, TILE_SIZE * 4.4));
const SIDE_LENGTH: usize = 4;

// fn range_prod() -> structs::Product<ops::Range<i32>, ops::Range<i32>> {
//     return iproduct!(0..SIDE_LENGTH as i32, 0..SIDE_LENGTH as i32);
// }

#[derive(Debug, Clone, PartialEq, Eq, Component)]
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

impl From<Position> for Transform {
    fn from(pos: Position) -> Self {
        return pos.to_transform(10.0);
    }
}

#[derive(Component)]
struct Tile(u64);

impl From<Tile> for Color {
    fn from(Tile(num): Tile) -> Self {
        match num {
            2 => Color::GOLD,
            4 => Color::ORANGE,
            8 => Color::ORANGE_RED,
            _ => Color::RED,
            // #TODO
        }
    }
}

fn create_tile(commands: &mut Commands, num: u64, position: Position) {
    commands
        .spawn_empty()
        .insert(Tile(num))
        .insert(position.clone())
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::from(Tile(num)),
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
) {
    for ev in move_evr.read() {
        // 移動方向と回転回数
        let (dx, dy, rot) = match ev {
            MoveEvent::Left => (-1, 0, 3),
            MoveEvent::Right => (1, 0, 1),
            MoveEvent::Up => (0, 1, 2),
            MoveEvent::Down => (0, -1, 0),
        };
        // 盤面の状態の取得
        let mut map: [[isize; SIDE_LENGTH]; SIDE_LENGTH] = [[0; SIDE_LENGTH]; SIDE_LENGTH];

        for (_, pos) in query.iter() {
            map[pos.x][pos.y] = 1;
        }

        for _ in 0..rot {
            rotate_map_ccw(&mut map);
        }

        for i in 0..SIDE_LENGTH {
            let mut v = 0;
            for j in 0..SIDE_LENGTH {
                v += 1 - map[i][j];
                map[i][j] += v - 1;
            }
        }

        for _ in rot..4 {
            rotate_map_ccw(&mut map);
        }

        for (mut trans, mut pos) in query.iter_mut() {
            let mv = map[pos.x][pos.y];
            pos.x = (pos.x as isize + dx * mv) as usize;
            pos.y = (pos.y as isize + dy * mv) as usize;
            *trans = pos.clone().into();
        }
    }
}

fn rotate_map_ccw(a: &mut [[isize; SIDE_LENGTH]; SIDE_LENGTH]) {
    let mut b: [[isize; SIDE_LENGTH]; SIDE_LENGTH] = [[0; SIDE_LENGTH]; SIDE_LENGTH];
    for (i, j) in iproduct!(0..SIDE_LENGTH, 0..SIDE_LENGTH) {
        b[i][j] = a[SIDE_LENGTH - j - 1][i];
    }
    *a = b;
}

#[derive(Component)]
struct Background;

fn create_board(commands: &mut Commands) {
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
    create_board(&mut commands);

    for (i, j, num) in [(1, 0, 2), (3, 0, 4), (1, 3, 8)] {
        create_tile(&mut commands, num, Position::new(i, j));
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
        .add_systems(Startup, setup)
        .add_event::<MoveEvent>()
        .add_systems(Update, send_move_event_from_keyboard)
        .add_systems(Update, send_move_event_from_touch)
        .add_systems(Update, move_tiles_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
