use bevy::{
    app::{AppExit, Plugin},
    ecs::{event::EventWriter, schedule::OnEnter},
};

use crate::state;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(state::App::Game(state::Game::GameOver)), end_game);
    }
}

pub fn end_game(mut exit: EventWriter<AppExit>) {
    println!("GAME OVER!!");
    exit.send(AppExit);
}
