use bevy::ecs::schedule::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Input,
    Calculate,
    Movement,
    Spawn,
    GameOver,
}
