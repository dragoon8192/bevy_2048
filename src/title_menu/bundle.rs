use bevy::prelude::*;
use strum::IntoEnumIterator;

use super::{
    component,
    constant::{color, font, layout},
};
use crate::constants::color::{BOARD_COLOR_0, BOARD_COLOR_1};
use crate::constants::layout as global_layout;

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
                    width: Val::Px(global_layout::WINDOW_WIDTH),
                    height: Val::Px(global_layout::WINDOW_HEIGHT),
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
                    width: Val::Px(layout::TITLE_WIDTH),
                    height: Val::Px(layout::TITLE_HEIGHT),
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
                    font_size: font::TITLE_SIZE,
                    color: color::TITLE_TEXT,
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
                    width: Val::Px(layout::MENU_WIDTH),
                    height: Val::Px(layout::MENU_HEIGHT),
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
                    width: Val::Px(layout::BUTTON_WIDTH),
                    height: Val::Px(layout::BUTTON_HEIGHT),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(layout::BUTTON_BORDER)),
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
                    color: color::MENU_TEXT,
                    font_size: font::MENU_SIZE,
                },
            ));
        };
    }
}
