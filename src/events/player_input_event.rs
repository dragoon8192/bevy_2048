use bevy::prelude::*;

#[derive(Event, PartialEq, Eq)]
pub enum PlayerInputEvent {
    Left,
    Right,
    Up,
    Down,
}

pub fn emit_player_input_event_from_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut input_evw: EventWriter<PlayerInputEvent>,
) {
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        input_evw.send(PlayerInputEvent::Left);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        input_evw.send(PlayerInputEvent::Right);
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        input_evw.send(PlayerInputEvent::Up);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        input_evw.send(PlayerInputEvent::Down);
    }
}
