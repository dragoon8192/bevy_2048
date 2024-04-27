use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Res},
};

use crate::{
    bundle::{background_board::create_background_board, tile::create_tile},
    components::{position::Position, tile::Tile},
    states::game_state::GameState,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<GameState>()
            .add_systems(Startup, setup)
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    create_background_board(&mut commands);
    for (i, j, num) in [(1, 0, 2), (3, 0, 4), (1, 3, 8)] {
        create_tile(&mut commands, &asset_server, Tile(num), Position::new(i, j));
    }
}
