use bevy::prelude::*;
use strum::IntoEnumIterator;

use super::{component, constant};
use crate::constants::{
    color::{BOARD_COLOR_0, BOARD_COLOR_1, MENU_TEXT_COLOR, TITLE_TEXT_COLOR},
    layout::{
        MAIN_BOARD_HEIGHT, MAIN_BOARD_WIDTH, MENU_BUTTON_BORDER, MENU_BUTTON_HEIGHT,
        MENU_BUTTON_WIDTH, SCORE_BOARD_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH,
    },
};

#[derive(Bundle)]
pub struct Screen {
    marker: component::Screen,
    node_bundle: NodeBundle,
}

impl Default for Screen {
    fn default() -> Self {
        return Self {
            marker: component::Screen,
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

impl Screen {
    pub fn child_builder(font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            let title_box = Title::default();
            let title_child = title_box.child_builder(font.clone());
            parent.spawn(title_box).with_children(title_child);
            let menu_box = MenuBox::default();
            let menu_child = menu_box.child_builder(font.clone());
            parent.spawn(menu_box).with_children(menu_child);
        };
    }
}

#[derive(Bundle)]
struct Title {
    node: NodeBundle,
}

impl Default for Title {
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

impl Title {
    fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            parent.spawn(TextBundle::from_section(
                "2048.rs",
                TextStyle {
                    font: font.clone(),
                    font_size: constant::font::TITLE_SIZE,
                    color: TITLE_TEXT_COLOR,
                },
            ));
        };
    }
}

#[derive(Bundle)]
struct MenuBox {
    node: NodeBundle,
}

impl Default for MenuBox {
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

impl MenuBox {
    fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            for action in component::ButtonAction::iter() {
                let button = MenuButton::new(action);
                let child_builder = button.child_builder(font.clone());
                parent.spawn(button).with_children(child_builder);
            }
        };
    }
}

#[derive(Bundle)]
struct MenuButton {
    action: component::ButtonAction,
    button: ButtonBundle,
}

impl Default for MenuButton {
    fn default() -> Self {
        return Self {
            action: component::ButtonAction::GameStart,
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

impl MenuButton {
    fn new(action: component::ButtonAction) -> Self {
        return Self {
            action,
            ..default()
        };
    }
    fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        let val = match self.action {
            component::ButtonAction::GameStart => "Start",
            component::ButtonAction::ScoreBoard => "Scores",
            component::ButtonAction::Quit => "Quit",
        };
        return move |parent| {
            parent.spawn(TextBundle::from_section(
                val,
                TextStyle {
                    font,
                    color: MENU_TEXT_COLOR,
                    font_size: constant::font::MENU_SIZE,
                },
            ));
        };
    }
}
