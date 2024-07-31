use bevy::prelude::*;
use strum_macros::EnumIter;

#[derive(Component)]
pub struct Screen;

#[derive(Component, EnumIter)]
pub enum ButtonAction {
    GameStart,
    ScoreBoard,
    Quit,
}
