use bevy::{ecs::query::QueryEntityError, prelude::*};

use crate::{
    components::{position::Position, score_text::ScoreText, tile::Tile},
    error::handle_query_entity_errors,
    plugins::calculate::TileMovementEvent,
    resources::score::Score,
    states::game_state::GameState,
};

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Movement),
            (
                move_tiles.pipe(handle_query_entity_errors),
                update_tiles.pipe(handle_query_entity_errors),
                update_score.pipe(handle_query_entity_errors),
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
    mut score: ResMut<Score>,
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
                score.add(tile0.num());
                commands.entity(e1).despawn_recursive();
            }
        }
    }
    return Ok(());
}

pub fn update_tiles(
    mut query_p: Query<
        (&Tile, &Position, &mut Transform, &mut Sprite, &Children),
        Or<(Changed<Tile>, Changed<Position>)>,
    >,
    mut query_c: Query<(&mut Transform, &mut Text), (With<Parent>, Without<Children>)>,
) -> Result<(), QueryEntityError> {
    dbg!("System: update_tiles");
    for (tile, pos, mut trans_p, mut sprite, children) in query_p.iter_mut() {
        *trans_p = (*pos).into();
        for child in children.iter() {
            let (mut trans_c, mut text) = query_c.get_mut(*child)?;
            text.sections[0].value = tile.to_string();
            trans_c.scale.x = 1.0 / tile.to_string().len() as f32;
            // text.sections[0].style.font_size = TILE_FONT_SIZE / tile.to_string().len() as f32;
        }
        sprite.color = (*tile).into();
    }
    return Ok(());
}

pub fn update_score(
    mut query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
) -> Result<(), QueryEntityError> {
    let mut text = query.single_mut();
    text.sections[0].value = score.to_string();
    return Ok(());
}
