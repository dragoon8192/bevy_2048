use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;

use super::calculate::TileMovementEvent;
use crate::components::position::Position;
use crate::components::tile::Tile;
use crate::constants::TILE_FONT_SIZE;
use crate::error::handle_query_entity_errors;
use crate::states::game_state::GameState;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Movement),
            (
                move_tiles.pipe(handle_query_entity_errors),
                update_tiles,
                GameState::Spawn.set_next(),
            )
                .chain(),
        );
    }
}

pub fn move_tiles(
    mut tile_move_evr: EventReader<TileMovementEvent>,
    mut query: Query<(&mut Position, &mut Tile)>,
    mut commands: Commands,
) -> Result<(), QueryEntityError> {
    dbg!("System: move_tiles");
    for ev in tile_move_evr.read() {
        match ev {
            &TileMovementEvent::OneStep(e, turn) => {
                let (mut pos, _) = query.get_mut(e)?;
                pos.shift(turn.downward_unit());
            }
            &TileMovementEvent::Merge(e0, e1, _) => {
                let (_, mut tile0) = query.get_mut(e0)?;
                tile0.double();
                commands.entity(e1).despawn();
            }
        }
    }
    return Ok(());
}

pub fn update_tiles(
    mut query: Query<
        (&Tile, &Position, &mut Transform, &mut Text, &mut Sprite),
        Or<(Changed<Tile>, Changed<Position>)>,
    >,
) {
    dbg!("System: update_tiles");
    for (tile, pos, mut trans, mut text, mut sprite) in query.iter_mut() {
        *trans = pos.clone().into();
        text.sections[0].value = tile.to_string();
        text.sections[0].style.font_size = TILE_FONT_SIZE / tile.to_string().len() as f32;
        sprite.color = tile.clone().into();
    }
}
