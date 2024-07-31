use bevy::prelude::*;

use crate::{
    bundle::{
        main_board::create_main_board, score_board::create_score_board, tile::TileSpawnEvent,
    },
    constant::color,
    resources::score::Score,
    state,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(color::BACKGROUND))
            .init_state::<state::App>()
            .add_event::<TileSpawnEvent>()
            .add_systems(
                Startup,
                (
                    setup,
                    create_main_board,
                    create_score_board,
                    state::App::TitleMenu.set_next(),
                ),
            )
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.init_resource::<Score>();
}
