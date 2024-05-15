use bevy::prelude::*;

#[derive(Resource)]
pub struct BestScore(usize);

impl BestScore {
    pub fn update(&mut self, post: usize) {
        let pre = self.0;
        *self = BestScore(pre.max(post));
    }
}

impl Default for BestScore {
    fn default() -> Self {
        return Self(0);
    }
}

impl ToString for BestScore {
    fn to_string(&self) -> String {
        return self.0.to_string();
    }
}
