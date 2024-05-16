use bevy::prelude::*;

use super::bundle;
use crate::states::game_state::GameState;

pub struct TitleMenuPlugin;

impl Plugin for TitleMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TitleMenu), create_title_menu);
        app.add_systems(OnExit(GameState::TitleMenu), ());
    }
}
