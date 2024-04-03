use bevy::prelude::*;

#[derive(Event, PartialEq, Eq)]
pub enum MoveEvent {
    Left,
    Right,
    Up,
    Down,
}

pub fn emit_move_event_from_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut move_evw: EventWriter<MoveEvent>,
) {
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        move_evw.send(MoveEvent::Left);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        move_evw.send(MoveEvent::Right);
    } else if keyboard.just_pressed(KeyCode::ArrowUp) {
        move_evw.send(MoveEvent::Up);
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        move_evw.send(MoveEvent::Down);
    }
}
