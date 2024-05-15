use bevy::prelude::*;

use crate::{
    components::main_menu::MainMenuScreen,
    constants::{
        color::{BOARD_COLOR_0, TITLE_TEXT_COLOR},
        font::{MAIN_FONT_NAME, TITLE_FONT_SIZE},
        layout::{
            MAIN_BOARD_HEIGHT, MAIN_BOARD_WIDTH, SCORE_BOARD_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH,
        },
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
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
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
                .spawn(TitleBoxBundle::default())
                .with_children(TitleBoxBundle::child_builder(font));
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

#[derive(Bundle)]
struct TitleBoxBundle {
    node: NodeBundle,
}

impl Default for TitleBoxBundle {
    fn default() -> Self {
        return Self {
            node: NodeBundle {
                style: Style {
                    width: Val::Px(MAIN_BOARD_WIDTH),
                    height: Val::Px(SCORE_BOARD_HEIGHT),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(BOARD_COLOR_0),
                ..default()
            },
        };
    }
}

impl TitleBoxBundle {
    fn child_builder(font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            parent.spawn(TextBundle::from_section(
                "2048.rs",
                TextStyle {
                    font: font.clone(),
                    font_size: TITLE_FONT_SIZE,
                    color: TITLE_TEXT_COLOR,
                    ..default()
                },
            ));
        };
    }
}

#[derive(Bundle)]
struct MenuBoxBundle {
    node: NodeBundle,
}

impl Default for MenuBoxBundle {
    fn default() -> Self {
        return Self {
            node: NodeBundle {
                style: Style {
                    width: Val::Px(MAIN_BOARD_WIDTH),
                    height: Val::Px(MAIN_BOARD_HEIGHT),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(BOARD_COLOR_0),
                ..default()
            },
        };
    }
}

impl MenuBoxBundle {
    fn child_builder(font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            let button_style = Style {
                width: Val::Px(todo!()),
                ..default()
            };
            parent.spawn(TextBundle::from_section(
                "2048.rs",
                TextStyle {
                    font: font.clone(),
                    font_size: TITLE_FONT_SIZE,
                    color: TITLE_TEXT_COLOR,
                    ..default()
                },
            ));
        };
    }
}

pub fn create_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(MAIN_FONT_NAME);
    commands
        .spawn(MainMenuScreenBundle::default())
        .with_children(MainMenuScreenBundle::child_builder(font));
}

pub fn remove_main_menu_screen(query: Query<Entity, With<MainMenuScreen>>, mut commands: Commands) {
    commands.entity(query.single()).despawn_recursive();
}