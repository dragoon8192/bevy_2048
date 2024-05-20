use bevy::{app::AppExit, prelude::*};

use super::{bundle, component, constant::font};
use crate::states::game_state::GameState;

pub fn create_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(font::NAME);
    commands
        .spawn(bundle::Screen::default())
        .with_children(bundle::Screen::child_builder(font));
}

pub fn remove_screen(query: Query<Entity, With<component::Screen>>, mut commands: Commands) {
    commands.entity(query.single()).despawn_recursive();
}

pub fn menu_action(
    query: Query<(&Interaction, &component::ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut exit: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button_action {
                component::ButtonAction::GameStart => {
                    game_state.set(GameState::Spawn);
                }
                component::ButtonAction::ScoreBoard => {}
                component::ButtonAction::Quit => {
                    exit.send(AppExit);
                }
            }
        }
    }
}
