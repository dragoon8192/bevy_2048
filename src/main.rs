use bevy::app::*;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use events::move_event::MoveEvent;
use states::game_state::return_to_move_state;
use states::game_state::GameState;
use systems::tile::create_random_tile;

mod components;
mod constants;
mod events;
mod states;
mod systems;
use crate::components::position::Position;
use crate::components::tile::Tile;
use crate::constants::WINDOW_SIZE;
use crate::events::move_event::emit_move_event_from_keyboard;
use crate::systems::background_board::create_background_board;
use crate::systems::tile::create_tile;
use crate::systems::tile::handle_tile_movement;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    create_background_board(&mut commands);

    for (i, j, num) in [(1, 0, 2), (3, 0, 4), (1, 3, 8)] {
        create_tile(&mut commands, Tile(num), Position::new(i, j));
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
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_systems(Startup, setup)
        .init_state::<GameState>()
        .add_event::<MoveEvent>()
        .add_systems(
            Update,
            (handle_tile_movement, emit_move_event_from_keyboard).run_if(in_state(GameState::Move)),
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
