use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;

use super::calculate::TileMovementEvent;
use crate::components::position::Position;
use crate::components::tile::Tile;
use crate::error::handle_query_entity_errors;
use crate::states::game_state::GameState;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Movement),
            (move_tiles.pipe(handle_query_entity_errors), update_tiles).chain(),
        );
    }
}

pub fn move_tiles(
    mut tile_move_evr: EventReader<TileMovementEvent>,
    mut query: Query<(&mut Position, &mut Transform, &mut Tile, &mut Text)>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) -> Result<(), QueryEntityError> {
    dbg!("System: move_tiles");
    for ev in tile_move_evr.read() {
        match ev {
            &TileMovementEvent::OneStep(e, turn) => {
                let (mut pos, mut trans, _, _) = query.get_mut(e)?;
                pos.shift(turn.downward_unit());
                *trans = pos.clone().into();
            }
            &TileMovementEvent::Merge(e0, e1, _) => {
                let (_, _, mut tile0, mut text) = query.get_mut(e0)?;
                tile0.double();
                text.sections[0].value = tile0.to_string();
                commands.entity(e1).despawn();
            }
        }
    }
    next_state.set(GameState::Spawn);
    dbg!(GameState::Spawn);
    return Ok(());
}

pub fn update_tiles(
    mut query: Query<
        (&Tile, &Position, &mut Transform, &mut Text, &mut Sprite),
        Or<(Changed<Tile>, Changed<Position>)>,
    >,
) {
    for (tile, pos, mut trans, mut text, mut sprite) in query.iter_mut() {
        *trans = pos.clone().into();
        text.sections[0].value = tile.to_string();
        sprite.color = tile.clone().into();
    }
}
