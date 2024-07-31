use bevy::prelude::*;

use crate::app_state;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputEvent>().add_systems(
            Update,
            input_from_keyboard.run_if(in_state(app_state::App::Game(app_state::Game::Input))),
        );
    }
}

#[derive(Event, PartialEq, Eq)]
pub enum PlayerInputEvent {
    Left,
    Right,
    Up,
    Down,
}

pub fn input_from_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut input_evw: EventWriter<PlayerInputEvent>,
    mut next_state: ResMut<NextState<app_state::App>>,
) {
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        input_evw.send(PlayerInputEvent::Left);
        next_state.set(app_state::App::Game(app_state::Game::Calculate));
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        input_evw.send(PlayerInputEvent::Right);
        next_state.set(app_state::App::Game(app_state::Game::Calculate));
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        input_evw.send(PlayerInputEvent::Up);
        next_state.set(app_state::App::Game(app_state::Game::Calculate));
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        input_evw.send(PlayerInputEvent::Down);
        next_state.set(app_state::App::Game(app_state::Game::Calculate));
    }
}
