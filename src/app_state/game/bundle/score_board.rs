use bevy::prelude::*;

use super::super::{
    component::score_board::ScoreBoard,
    component::score_text::ScoreText,
    constant::{color, font, layout},
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
                background_color: BackgroundColor(color::board::MAIN),
                ..default()
            },
        };
    }
}

impl ScoreBoardBundle {
    fn child_builder(font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        let style = TextStyle {
            font: font.clone(),
            font_size: font::size::SUB,
            color: color::text::MAIN,
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
