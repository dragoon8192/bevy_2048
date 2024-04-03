use bevy::ecs::query;
use bevy::ecs::system::QueryLens;
use bevy::prelude::*;

use crate::components::position::Position;
use crate::components::tile::Tile;
use crate::systems::position::check_positions_are_full;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Move,
    Spawn,
    GameOver,
}

pub fn check_and_set_game_over_state(
    mut query: Query<&Position, With<Tile>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut lens: QueryLens<&Position> = query.transmute_lens();
    if check_positions_are_full(&mut lens) {
        next_state.set(GameState::GameOver);
    } else {
        next_state.set(GameState::Move);
    }
}
