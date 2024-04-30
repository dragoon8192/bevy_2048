use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(usize);

impl Default for Score {
    fn default() -> Self {
        return Self(0);
    }
}
