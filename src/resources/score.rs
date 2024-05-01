use std::ops::AddAssign;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(usize);

impl Score {
    pub fn add(&mut self, val: usize) {
        *self += Score(val);
    }
}

impl Default for Score {
    fn default() -> Self {
        return Self(0);
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0);
    }
}

impl ToString for Score {
    fn to_string(&self) -> String {
        return self.0.to_string();
    }
}
