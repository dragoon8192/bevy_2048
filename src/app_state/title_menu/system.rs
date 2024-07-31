use bevy::{app::AppExit, prelude::*};

use super::{bundle, component, constant::font};
use crate::app_state;

pub fn create_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(font::BODY.font);
    commands
        .spawn(bundle::Screen::default())
        .with_children(bundle::Screen::child_builder(font));
}

pub fn menu_action(
    query: Query<(&Interaction, &component::ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut exit: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<app_state::App>>,
) {
    for (interaction, button_action) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button_action {
                component::ButtonAction::GameStart => {
                    game_state.set(app_state::App::Game(app_state::Game::Spawn));
                }
                component::ButtonAction::ScoreBoard => {
                    game_state.set(app_state::App::ScoreBoard);
                }
                component::ButtonAction::Quit => {
                    exit.send(AppExit);
                }
            }
        }
    }
}

pub fn remove_screen(query: Query<Entity, With<component::Screen>>, mut commands: Commands) {
    commands.entity(query.single()).despawn_recursive();
}
