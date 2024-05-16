use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    components::title_menu,
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
struct ScreenBundle {
    marker: title_menu::Screen,
    node_bundle: NodeBundle,
}

impl Default for ScreenBundle {
    fn default() -> Self {
        return Self {
            marker: title_menu::Screen,
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

impl ScreenBundle {
    fn child_builder(font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            let title_box = TitleBundle::default();
            let title_child = title_box.child_builder(font.clone());
            parent.spawn(title_box).with_children(title_child);
            let menu_box = MenuBundle::default();
            let menu_child = menu_box.child_builder(font.clone());
            parent.spawn(menu_box).with_children(menu_child);
        };
    }
}

#[derive(Bundle)]
struct TitleBundle {
    node: NodeBundle,
}

impl Default for TitleBundle {
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

impl TitleBundle {
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
struct MenuBundle {
    node: NodeBundle,
}

impl Default for MenuBundle {
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

impl MenuBundle {
    fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            for action in title_menu::ButtonAction::iter() {
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

pub fn create_title_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(MAIN_FONT_NAME);
    commands
        .spawn(ScreenBundle::default())
        .with_children(ScreenBundle::child_builder(font));
}

pub fn remove_main_menu_screen(
    query: Query<Entity, With<TitleMenuScreen>>,
    mut commands: Commands,
) {
    commands.entity(query.single()).despawn_recursive();
}
