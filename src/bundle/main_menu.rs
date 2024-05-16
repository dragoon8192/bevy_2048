use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    components::{main_menu::MainMenuScreen, menu_button_action::MenuButtonAction},
    constants::{
        color::{BOARD_COLOR_0, BOARD_COLOR_1, MENU_TEXT_COLOR, TITLE_TEXT_COLOR},
        font::{MAIN_FONT_NAME, MENU_FONT_SIZE, TITLE_FONT_SIZE},
        layout::{
            MAIN_BOARD_HEIGHT, MAIN_BOARD_WIDTH, MENU_BUTTON_BORDER, MENU_BUTTON_HEIGHT,
            MENU_BUTTON_WIDTH, SCORE_BOARD_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH,
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
            let title_box = TitleBoxBundle::default();
            let title_child = title_box.child_builder(font.clone());
            parent.spawn(title_box).with_children(title_child);
            let menu_box = MenuBoxBundle::default();
            let menu_child = menu_box.child_builder(font.clone());
            parent.spawn(menu_box).with_children(menu_child);
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
    fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            parent.spawn(TextBundle::from_section(
                "2048.rs",
                TextStyle {
                    font: font.clone(),
                    font_size: TITLE_FONT_SIZE,
                    color: TITLE_TEXT_COLOR,
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
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                background_color: BackgroundColor(BOARD_COLOR_0),
                ..default()
            },
        };
    }
}

impl MenuBoxBundle {
    fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            for action in MenuButtonAction::iter() {
                let button = MenuButtonBundle::new(action);
                let child_builder = button.child_builder(font.clone());
                parent.spawn(button).with_children(child_builder);
            }
        };
    }
}

#[derive(Bundle)]
struct MenuButtonBundle {
    action: MenuButtonAction,
    button: ButtonBundle,
}

impl Default for MenuButtonBundle {
    fn default() -> Self {
        return Self {
            action: MenuButtonAction::GameStart,
            button: ButtonBundle {
                style: Style {
                    width: Val::Px(MENU_BUTTON_WIDTH),
                    height: Val::Px(MENU_BUTTON_HEIGHT),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(MENU_BUTTON_BORDER)),
                    ..default()
                },
                background_color: BOARD_COLOR_0.into(),
                border_color: BOARD_COLOR_1.into(),
                ..default()
            },
        };
    }
}

impl MenuButtonBundle {
    fn new(action: MenuButtonAction) -> Self {
        return Self {
            action,
            ..default()
        };
    }
    fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        let val = match self.action {
            MenuButtonAction::GameStart => "Start",
            MenuButtonAction::ScoreBoard => "Scores",
            MenuButtonAction::Quit => "Quit",
        };
        return move |parent| {
            parent.spawn(TextBundle::from_section(
                val,
                TextStyle {
                    font,
                    color: MENU_TEXT_COLOR,
                    font_size: MENU_FONT_SIZE,
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
