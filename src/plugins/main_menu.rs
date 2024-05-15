use bevy::prelude::*;

use crate::{bundle::main_menu::create_main_menu, states::game_state::GameState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), create_main_menu);
    }
}
