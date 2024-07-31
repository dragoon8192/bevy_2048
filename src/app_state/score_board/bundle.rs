use bevy::prelude::*;

use super::constant::{color, font, layout};

#[derive(Bundle)]
pub struct Screen {
    node_bundle: NodeBundle,
}

impl Default for Screen {
    fn default() -> Self {
        return Self {
            node_bundle: NodeBundle {
                style: Style {
                    width: Val::Px(layout::WINDOW.width),
                    height: Val::Px(layout::WINDOW.height),
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
            let header_box = Header::default();
            let header_child = header_box.child_builder(font.clone());
            parent.spawn(header_box).with_children(header_child);
            let body_box = Body::default();
            let body_child = body_box.child_builder(font.clone());
            parent.spawn(body_box).with_children(body_child);
        };
    }
}

#[derive(Bundle)]
struct Header {
    node: NodeBundle,
}

impl Default for Header {
    fn default() -> Self {
        return Self {
            node: NodeBundle {
                style: Style {
                    width: Val::Px(layout::HEADER.width),
                    height: Val::Px(layout::HEADER.height),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(color::background::MAIN),
                ..default()
            },
        };
    }
}

impl Header {
    fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {
            parent.spawn(TextBundle::from_section(
                "2048.rs",
                TextStyle {
                    font: font.clone(),
                    font_size: font::HEADER.font_size.into(),
                    color: color::text::HEADER,
                },
            ));
        };
    }
}

#[derive(Bundle)]
struct Body {
    node: NodeBundle,
}

impl Default for Body {
    fn default() -> Self {
        return Self {
            node: NodeBundle {
                style: Style {
                    width: Val::Px(layout::BODY.width),
                    height: Val::Px(layout::BODY.height),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                background_color: BackgroundColor(color::background::MAIN),
                ..default()
            },
        };
    }
}

impl Body {
    fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
        return move |parent| {};
    }
}

// #[derive(Bundle)]
// struct MenuButton {
//     action: component::ButtonAction,
//     button: ButtonBundle,
// }

// impl Default for MenuButton {
//     fn default() -> Self {
//         return Self {
//             action: component::ButtonAction::GameStart,
//             button: ButtonBundle {
//                 style: Style {
//                     width: Val::Px(layout::BUTTON_WIDTH),
//                     height: Val::Px(layout::BUTTON_HEIGHT),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     border: UiRect::all(Val::Px(layout::BUTTON_BORDER)),
//                     ..default()
//                 },
//                 background_color: BOARD_COLOR_0.into(),
//                 border_color: BOARD_COLOR_1.into(),
//                 ..default()
//             },
//         };
//     }
// }

// impl MenuButton {
//     fn new(action: component::ButtonAction) -> Self {
//         return Self {
//             action,
//             ..default()
//         };
//     }
//     fn child_builder(&self, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
//         let val = match self.action {
//             component::ButtonAction::GameStart => "Start",
//             component::ButtonAction::ScoreBoard => "Scores",
//             component::ButtonAction::Quit => "Quit",
//         };
//         return move |parent| {
//             parent.spawn(TextBundle::from_section(
//                 val,
//                 TextStyle {
//                     font,
//                     color: color::body_TEXT,
//                     font_size: font::body_SIZE,
//                 },
//             ));
//         };
//     }
// }
