use bevy::ecs::system::{Query, QueryLens};
use itertools::iproduct;
use std::collections::BTreeSet;

use crate::components::position::Position;
use crate::constants::SIDE_LENGTH;

fn get_positions_set(lens: &mut QueryLens<&Position>) -> BTreeSet<Position> {
    let query: Query<'_, '_, &Position> = lens.query();
    let iter = query.iter().cloned();
    return BTreeSet::from_iter(iter);
}

fn positions_univ_set() -> BTreeSet<Position> {
    return BTreeSet::from_iter(iproduct!(0..SIDE_LENGTH, 0..SIDE_LENGTH).map(Position::from));
}

pub fn get_positions_complement_set(lens: &mut QueryLens<&Position>) -> BTreeSet<Position> {
    let query: Query<'_, '_, &Position> = lens.query();
    let iter = query.iter().cloned();
    let set: BTreeSet<Position> = BTreeSet::from_iter(iter);
    return positions_univ_set().difference(&set).cloned().collect();
}
