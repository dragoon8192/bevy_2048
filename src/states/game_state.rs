use bevy::ecs::{
    schedule::{NextState, States},
    system::ResMut,
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    TitleMenu,
    Spawn,
    Input,
    Calculate,
    Movement,
    GameOver,
}

impl GameState {
    pub fn set_next(self) -> impl FnMut(ResMut<NextState<GameState>>) {
        return move |mut next: ResMut<NextState<GameState>>| {
            dbg!(self);
            next.set(self);
        };
    }
}
