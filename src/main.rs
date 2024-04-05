use bevy::app::*;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;

mod components;
mod constants;
mod events;
mod states;
mod systems;

use components::position::Position;
use components::tile::Tile;
use constants::WINDOW_SIZE;
use events::move_event::emit_move_event_from_keyboard;
use events::move_event::MoveEvent;
use states::game_state::check_and_set_game_over_state;
use states::game_state::end_game;
use states::game_state::GameState;
use systems::background_board::create_background_board;
use systems::tile::create_random_tile;
use systems::tile::create_tile;
use systems::tile::handle_tile_movement;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    create_background_board(&mut commands);

    for (i, j, num) in [(1, 0, 2), (3, 0, 4), (1, 3, 8)] {
        create_tile(&mut commands, &asset_server, Tile(num), Position::new(i, j));
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
            (create_random_tile, check_and_set_game_over_state)
                .chain()
                .run_if(in_state(GameState::Spawn)),
        )
        .add_systems(OnEnter(GameState::GameOver), end_game)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
