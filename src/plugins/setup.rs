use bevy::prelude::*;

use crate::{
    bundle::{
        main_board::create_main_board, score_board::create_score_board, tile::TileSpawnEvent,
    },
    constants::color::BACKGROUND_COLOR,
    resources::score::Score,
    states::game_state::GameState,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .init_state::<GameState>()
            .add_event::<TileSpawnEvent>()
            .add_systems(
                Startup,
                (
                    setup,
                    create_main_board,
                    create_score_board,
                    GameState::TitleMenu.set_next(),
                ),
            )
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.init_resource::<Score>();
}
