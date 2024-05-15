use bevy::prelude::*;

use crate::{
    components::main_menu::MainMenuScreen,
    constants::{BOARD_COLOR_0, WINDOW_HEIGHT, WINDOW_WIDTH},
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
                    ..default()
                },
                ..default()
            },
        };
    }
}

pub fn create_main_menu(mut commands: Commands) {
    commands.spawn(MainMenuScreenBundle::default());
}

pub fn remove_main_menu_screen(query: Query<Entity, With<MainMenuScreen>>, mut commands: Commands) {
    commands.entity(query.single()).despawn_recursive();
}
