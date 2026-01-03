use crate::domain::board::{Board, Score};

pub struct EvaluateGameEndUseCase {}

impl EvaluateGameEndUseCase {

    pub fn execute(&self, board: &Board) -> Option<Score> {
        board.end_of_game()
    }

}