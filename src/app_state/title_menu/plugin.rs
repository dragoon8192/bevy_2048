use bevy::prelude::*;

use super::system;
use crate::app_state;

pub struct TitleMenuPlugin;

impl Plugin for TitleMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app_state::App::TitleMenu), system::create_screen);
        app.add_systems(
            Update,
            (system::menu_action).run_if(in_state(app_state::App::TitleMenu)),
        );
        app.add_systems(OnExit(app_state::App::TitleMenu), system::remove_screen);
    }
}
