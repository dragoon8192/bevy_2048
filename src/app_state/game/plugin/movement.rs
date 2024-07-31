use bevy::{ecs::query::QueryEntityError, prelude::*};

use super::super::{
    component::{position::Position, score_text::ScoreText, tile::Tile},
    plugin::calculate::TileMovementEvent,
    resource::score,
};
use crate::{app_state, error::handle_query_entity_errors};

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(app_state::App::Game(app_state::Game::Movement)),
            (
                move_tiles.pipe(handle_query_entity_errors),
                update_tiles.pipe(handle_query_entity_errors),
                update_score.pipe(handle_query_entity_errors),
                app_state::App::Game(app_state::Game::Spawn).set_next(),
            )
                .chain(),
        );
    }
}

pub fn move_tiles(
    mut tile_move_evr: EventReader<TileMovementEvent>,
    mut query: Query<(&mut Position, &mut Tile)>,
    mut commands: Commands,
    mut score: ResMut<score::Score>,
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
    score: Res<score::Score>,
) -> Result<(), QueryEntityError> {
    let mut text = query.single_mut();
    text.sections[0].value = score.to_string();
    return Ok(());
}
