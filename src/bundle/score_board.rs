use bevy::prelude::*;

use crate::{
    components::score_board::ScoreBoard,
    components::score_text::ScoreText,
    constants::{
        BOARD_COLOR_0, MAIN_BOARD_WIDTH, SCORE_BOARD_HEIGHT, SCORE_FONT_SIZE, SCORE_TEXT_COLOR,
    },
};

#[derive(Bundle)]
struct ScoreBoardBundle {
    marker: ScoreBoard,
    node_bundle: NodeBundle,
}

#[derive(Bundle)]
struct ScoreTextBundle {
    marker: ScoreText,
    text_bundle: TextBundle,
}

impl Default for ScoreBoardBundle {
    fn default() -> Self {
        return Self {
            marker: ScoreBoard,
            node_bundle: NodeBundle {
                style: Style {
                    width: Val::Px(MAIN_BOARD_WIDTH),
                    height: Val::Px(SCORE_BOARD_HEIGHT),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                background_color: BackgroundColor(BOARD_COLOR_0),
                ..default()
            },
        };
    }
}

impl ScoreBoardBundle {
    fn child_builder(font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        let style = TextStyle {
            font: font.clone(),
            font_size: SCORE_FONT_SIZE,
            color: SCORE_TEXT_COLOR,
            ..default()
        };
        return move |parent| {
            parent.spawn(TextBundle::from_section("score:", style.clone()));
            parent.spawn(ScoreTextBundle {
                marker: ScoreText,
                text_bundle: TextBundle::from_section("0", style.clone()),
            });
        };
    }
}

pub fn create_score_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Kenney Space.ttf");
    commands
        .spawn(ScoreBoardBundle::default())
        .with_children(ScoreBoardBundle::child_builder(font));
}
