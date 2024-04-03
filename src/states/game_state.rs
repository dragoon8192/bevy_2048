use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Move,
    Spawn,
    GameOver,
}

pub fn return_to_move_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Move);
}
