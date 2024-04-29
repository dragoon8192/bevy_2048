use bevy::prelude::*;

#[derive(Resource)]
struct Score(usize);

impl Default for Score {
    fn default() -> Self {
        return Self(0);
    }
}
