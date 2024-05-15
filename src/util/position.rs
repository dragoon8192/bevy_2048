use bevy::ecs::query::With;
use bevy::ecs::system::{Query, QueryLens};
use itertools::iproduct;
use std::collections::BTreeSet;

use crate::components::{position::Position, tile::Tile};
use crate::constants::layout::{GRID_HEIGHT, GRID_WIDTH};

fn get_positions_set(mut lens: QueryLens<&Position>) -> BTreeSet<Position> {
    let query: Query<'_, '_, &Position> = lens.query();
    return BTreeSet::from_iter(query.iter().cloned());
}

fn generate_universal_position_set() -> BTreeSet<Position> {
    return BTreeSet::from_iter(iproduct!(0..GRID_WIDTH, 0..GRID_HEIGHT).map(Position::from));
}

pub fn board_is_full(mut query: Query<&Position, With<Tile>>) -> bool {
    let positions_set = get_positions_set(query.transmute_lens());
    let univ_set = generate_universal_position_set();
    return positions_set == univ_set;
}

pub fn get_positions_complement_set(mut lens: QueryLens<&Position>) -> BTreeSet<Position> {
    let query: Query<'_, '_, &Position> = lens.query();
    let iter = query.iter().cloned();
    let set: BTreeSet<Position> = BTreeSet::from_iter(iter);
    return generate_universal_position_set()
        .difference(&set)
        .cloned()
        .collect();
}
