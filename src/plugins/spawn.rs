use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;
use std::collections::BTreeSet;

use crate::bundle::tile::{spawn_tiles, TileSpawnEvent};
use crate::components::position::Position;
use crate::components::tile::Tile;
use crate::states::game_state::GameState;
use crate::util::position::{board_is_full, get_positions_complement_set};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Spawn),
            (
                (create_random_tile, spawn_tiles, GameState::Input.set_next())
                    .chain()
                    .run_if(not(board_is_full)),
                GameState::GameOver.set_next().run_if(board_is_full),
            ),
        );
    }
}

// 空いた Position への Tile の追加
pub fn create_random_tile(
    mut query: Query<&Position, With<Tile>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut tile_spawn_evw: EventWriter<TileSpawnEvent>,
) {
    dbg!("System: create_random_tile");
    let candidates_of_positions: BTreeSet<Position> =
        get_positions_complement_set(query.transmute_lens());
    let rnd_n = rng.next_u64() as usize % candidates_of_positions.len();
    let position = candidates_of_positions
        .iter()
        .nth(rnd_n)
        .expect("candidates_of_positions: out of range!!")
        .clone();
    let rnd_0_or_1 = rng.next_u32() % 2;
    let tile = Tile(2usize.pow(rnd_0_or_1 + 1));
    tile_spawn_evw.send(TileSpawnEvent { tile, position });
}
