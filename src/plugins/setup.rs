use bevy::{
    app::{Plugin, Startup, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::Commands,
};
use itertools::Itertools;

use crate::{
    bundle::{background_board::create_background_board, score_board::create_score_board},
    plugins::spawn::create_random_tile,
    states::game_state::GameState,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let tuple: (_, _, _) = [create_random_tile; 3].iter().collect_tuple().unwrap();
        app.init_state::<GameState>()
            .add_systems(
                Startup,
                (setup, create_background_board, create_score_board, tuple),
            )
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
