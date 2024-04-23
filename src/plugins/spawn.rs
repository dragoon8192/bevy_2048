use bevy::ecs::system::{Query, QueryLens};
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;
use std::collections::BTreeSet;

use crate::components::position::Position;
use crate::components::tile::Tile;
use crate::states::game_state::GameState;
use crate::util::position::{check_positions_are_full, get_positions_complement_set};
use crate::util::tile::create_tile;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Spawn),
            (create_random_tile, check_and_set_game_over_state).chain(),
        );
    }
}

// 空いた Position への Tile の追加
pub fn create_random_tile(
    mut commands: Commands,
    mut query: Query<&Position, With<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    asset_server: Res<AssetServer>,
) {
    let lens: QueryLens<&Position> = query.transmute_lens::<&Position>();
    let candidates_of_positions: BTreeSet<Position> = get_positions_complement_set(lens);
    let rnd_n = rng.next_u64() as usize % candidates_of_positions.len();
    let position = candidates_of_positions
        .iter()
        .nth(rnd_n)
        .expect("candidates_of_positions: out of range!!")
        .clone();
    let tile = Tile(2);
    create_tile(&mut commands, &asset_server, tile, position);
}

pub fn check_and_set_game_over_state(
    mut query: Query<&Position, With<Tile>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let lens: QueryLens<&Position> = query.transmute_lens();
    if check_positions_are_full(lens) {
        next_state.set(GameState::GameOver);
    } else {
        next_state.set(GameState::Input);
    }
}
