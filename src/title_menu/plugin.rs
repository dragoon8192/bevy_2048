use bevy::prelude::*;

use super::system;
use crate::states::game_state::GameState;

pub struct TitleMenuPlugin;

impl Plugin for TitleMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TitleMenu), system::create_screen);
        app.add_systems(
            Update,
            (system::menu_action).run_if(in_state(GameState::TitleMenu)),
        );
        app.add_systems(OnExit(GameState::TitleMenu), system::remove_screen);
    }
}
