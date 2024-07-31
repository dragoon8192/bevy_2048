use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;
use std::collections::BTreeSet;

use super::super::{
    bundle::tile::{spawn_tiles, TileSpawnEvent},
    component::{position::Position, tile::Tile},
    util::position::{board_is_full, get_positions_complement_set},
};

use crate::app_state;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(app_state::App::Game(app_state::Game::Spawn)),
            (
                (
                    create_random_tile,
                    spawn_tiles,
                    app_state::App::Game(app_state::Game::Input).set_next(),
                )
                    .chain()
                    .run_if(not(board_is_full)),
                app_state::App::Game(app_state::Game::GameOver)
                    .set_next()
                    .run_if(board_is_full),
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
    let rnd_n = rng.next_u32() as usize % candidates_of_positions.len();
    let position = candidates_of_positions
        .into_iter()
        .nth(rnd_n)
        .expect("candidates_of_positions: out of range!!");
    let rnd_1_or_2 = 1_u8 + (rng.next_u32() % 2) as u8;
    let tile = Tile(rnd_1_or_2);
    tile_spawn_evw.send(TileSpawnEvent { tile, position });
}
