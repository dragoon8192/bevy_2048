use bevy::prelude::*;
use itertools::{structs, Itertools};
use std::ops;

const WINDOW_SIZE: f32 = 500.0;
const TILE_SIZE: f32 = 60.0;
const TILE_SIZE_2D: Option<Vec2> = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
const BOARD_SIZE_2D: Option<Vec2> = Some(Vec2::new(TILE_SIZE * 4.4, TILE_SIZE * 4.4));
const SIDE_LENGTH: usize = 4;

fn range_prod() -> structs::Product<ops::Range<i32>, ops::Range<i32>> {
    return (0..SIDE_LENGTH as i32).cartesian_product(0..SIDE_LENGTH as i32);
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
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

fn create_tile(commands: &mut Commands, num: u64, position: Position) {
    commands
        .spawn_empty()
        .insert(Tile(num))
        .insert(position.clone())
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: TILE_SIZE_2D,
                ..Default::default()
            },
            transform: position.into(),
            ..Default::default()
        });
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
    for (i, j) in range_prod() {
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

    for (i, j) in range_prod() {
        create_tile(&mut commands, 2, Position::new(i, j));
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
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
