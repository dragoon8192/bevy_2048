use bevy::{
    app::{Plugin, Startup, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{schedule::IntoSystemConfigs, system::Commands},
};

use crate::{
    bundle::{
        background_board::create_background_board,
        score_board::create_score_board,
        tile::{spawn_tiles, TileSpawnEvent},
    },
    plugins::spawn::create_random_tile,
    resources::score::Score,
    states::game_state::GameState,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<GameState>()
            .add_event::<TileSpawnEvent>()
            .add_systems(
                Startup,
                (
                    setup,
                    create_background_board,
                    create_score_board,
                    (create_random_tile, spawn_tiles).chain(),
                ),
            )
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.init_resource::<Score>();
}
