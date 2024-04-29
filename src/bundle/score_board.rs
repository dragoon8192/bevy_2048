use bevy::prelude::*;

use crate::{
    components::score_board::ScoreBoard,
    constants::{
        BOARD_COLOR_0, MAIN_AND_SCORE_BOARD_MARGIN, MAIN_BOARD_HEIGHT, SCORE_BOARD_HEIGHT,
        SCORE_BOARD_SIZE_2D, SCORE_FONT_SIZE, SCORE_TEXT_COLOR,
    },
};

#[derive(Bundle)]
struct ScoreBoardBundle {
    score_board: ScoreBoard,
    sprite_bundle: SpriteBundle,
}

impl Default for ScoreBoardBundle {
    fn default() -> Self {
        return Self {
            score_board: ScoreBoard,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: BOARD_COLOR_0,
                    custom_size: SCORE_BOARD_SIZE_2D,
                    ..default()
                },
                transform: Transform::from_xyz(
                    0.0,
                    MAIN_BOARD_HEIGHT / 2.0
                        + SCORE_BOARD_HEIGHT / 2.0
                        + MAIN_AND_SCORE_BOARD_MARGIN,
                    0.0,
                ),
                ..default()
            },
        };
    }
}

impl ScoreBoardBundle {
    fn child_builder(font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    "score 0",
                    TextStyle {
                        font,
                        font_size: SCORE_FONT_SIZE,
                        color: SCORE_TEXT_COLOR,
                        ..default()
                    },
                ),
                ..default()
            });
        };
    }
}

pub fn create_score_board(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/Kenney Space.ttf");
    commands
        .spawn(ScoreBoardBundle::default())
        .with_children(ScoreBoardBundle::child_builder(font));
}
