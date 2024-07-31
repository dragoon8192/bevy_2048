use bevy::prelude::*;

use super::system;
use crate::state;

pub struct TitleMenuPlugin;

impl Plugin for TitleMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(state::App::TitleMenu), system::create_screen);
        app.add_systems(
            Update,
            (system::menu_action).run_if(in_state(state::App::TitleMenu)),
        );
        app.add_systems(OnExit(state::App::TitleMenu), system::remove_screen);
    }
}
