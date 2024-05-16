use bevy::prelude::*;

use crate::{bundle::main_menu::create_title_menu, states::game_state::GameState};

pub struct TitleMenuPlugin;

impl Plugin for TitleMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), create_title_menu);
        app.add_systems(OnExit(GameState::MainMenu), 
    }
}
