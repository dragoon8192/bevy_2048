use bevy::prelude::*;
use strum_macros::EnumIter;

#[derive(Component, EnumIter)]
pub enum MenuButtonAction {
    GameStart,
    ScoreBoard,
    Quit,
}
