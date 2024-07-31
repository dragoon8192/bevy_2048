use bevy::prelude::*;

use super::super::{
    bundle::{
        main_board::create_main_board, score_board::create_score_board, tile::TileSpawnEvent,
    },
    constant::color,
    resource::score::Score,
};
use crate::app_state;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(color::background::SUB))
            .init_state::<app_state::App>()
            .add_event::<TileSpawnEvent>()
            .add_systems(
                Startup,
                (
                    setup,
                    create_main_board,
                    create_score_board,
                    app_state::App::TitleMenu.set_next(),
                ),
            )
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.init_resource::<Score>();
}
