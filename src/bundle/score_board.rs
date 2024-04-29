#[derive(Bundle)]
struct ScoreBoardBundle {
    score_board: ScoreBoard,
}

impl Default for ScoreBoardBundle {
    fn default() -> Self {
        return Self {
            score_board: ScoreBoard,
        };
    }
}
