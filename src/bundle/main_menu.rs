use bevy::prelude::*;

use crate::{
    components::main_menu::MainMenuScreen,
    constants::{
        BOARD_COLOR_0, MAIN_BOARD_HEIGHT, MAIN_BOARD_WIDTH, SCORE_BOARD_HEIGHT, TITLE_FONT_SIZE,
        TITLE_TEXT_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH,
    },
};

#[derive(Bundle)]
struct MainMenuScreenBundle {
    marker: MainMenuScreen,
    node_bundle: NodeBundle,
}

impl Default for MainMenuScreenBundle {
    fn default() -> Self {
        return Self {
            marker: MainMenuScreen,
            node_bundle: NodeBundle {
                style: Style {
                    width: Val::Px(WINDOW_WIDTH),
                    height: Val::Px(WINDOW_HEIGHT),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        };
    }
}

impl MainMenuScreenBundle {
    fn child_builder(font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(MAIN_BOARD_WIDTH),
                        height: Val::Px(SCORE_BOARD_HEIGHT),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(BOARD_COLOR_0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "2048.rs",
                        TextStyle {
                            font: font.clone(),
                            font_size: TITLE_FONT_SIZE,
                            color: TITLE_TEXT_COLOR,
                            ..default()
                        },
                    ));
                });
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(MAIN_BOARD_WIDTH),
                    height: Val::Px(MAIN_BOARD_HEIGHT),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(BOARD_COLOR_0),
                ..default()
            });
        };
    }
}

pub fn create_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Kenney Space.ttf");
    commands
        .spawn(MainMenuScreenBundle::default())
        .with_children(MainMenuScreenBundle::child_builder(font));
}

pub fn remove_main_menu_screen(query: Query<Entity, With<MainMenuScreen>>, mut commands: Commands) {
    commands.entity(query.single()).despawn_recursive();
}
