use bevy::{
    app::{AppExit, Plugin},
    ecs::{event::EventWriter, schedule::OnEnter},
};

use crate::states::game_state::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::GameOver), end_game);
    }
}

pub fn end_game(mut exit: EventWriter<AppExit>) {
    println!("GAME OVER!!");
    exit.send(AppExit);
}
