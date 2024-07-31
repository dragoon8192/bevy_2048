pub mod game;
pub mod score_board;
pub mod title_menu;

use bevy::ecs::{
    schedule::{NextState, States},
    system::ResMut,
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum App {
    #[default]
    TitleMenu,
    ScoreBoard,
    Game(Game),
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum Game {
    #[default]
    Spawn,
    Input,
    Calculate,
    Movement,
    GameOver,
}

impl App {
    pub fn set_next(self) -> impl FnMut(ResMut<NextState<App>>) {
        return move |mut next: ResMut<NextState<App>>| {
            dbg!(self);
            next.set(self);
        };
    }
}
