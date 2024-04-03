use bevy::ecs::system::{Query, QueryLens};
use itertools::iproduct;
use std::collections::BTreeSet;

use crate::components::position::Position;
use crate::constants::SIDE_LENGTH;

fn get_positions_set(mut lens: QueryLens<&Position>) -> BTreeSet<Position> {
    let query: Query<'_, '_, &Position> = lens.query();
    let iter = query.iter().cloned();
    return BTreeSet::from_iter(iter);
}

fn generate_universal_position_set() -> BTreeSet<Position> {
    return BTreeSet::from_iter(iproduct!(0..SIDE_LENGTH, 0..SIDE_LENGTH).map(Position::from));
}

pub fn check_positions_are_full(lens: QueryLens<&Position>) -> bool {
    let positions_set = get_positions_set(lens);
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
