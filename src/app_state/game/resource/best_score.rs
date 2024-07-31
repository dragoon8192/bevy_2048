use crate::app_state::score_board::constant::SCORE_BOARD_NUM;
use bevy::prelude::*;

#[derive(Resource)]
pub struct BestScores(Vec<usize>);

impl BestScores {
    pub fn update(&mut self, score: usize) {
        let vec: &mut Vec<usize> = &mut self.0;
        vec.push(score);
        vec.sort();
        vec.reverse();
        vec.resize(SCORE_BOARD_NUM, 0);
    }
}

impl Default for BestScores {
    fn default() -> Self {
        return Self(vec![0; SCORE_BOARD_NUM]);
    }
}
