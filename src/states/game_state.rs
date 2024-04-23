use bevy::app::AppExit;
use bevy::prelude::*;

use crate::systems::position::check_positions_are_full;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Input,
    Calculate,
    Movement,
    Spawn,
    GameOver,
}

pub fn end_game(mut exit: EventWriter<AppExit>) {
    println!("GAME OVER!!");
    exit.send(AppExit);
}
