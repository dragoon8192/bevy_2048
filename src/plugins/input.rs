use bevy::prelude::*;

use crate::states::game_state::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputEvent>().add_systems(
            Update,
            input_from_keyboard.run_if(in_state(GameState::Input)),
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
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        input_evw.send(PlayerInputEvent::Left);
        next_state.set(GameState::Calculate);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        input_evw.send(PlayerInputEvent::Right);
        next_state.set(GameState::Calculate);
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        input_evw.send(PlayerInputEvent::Up);
        next_state.set(GameState::Calculate);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        input_evw.send(PlayerInputEvent::Down);
        next_state.set(GameState::Calculate);
    }
}