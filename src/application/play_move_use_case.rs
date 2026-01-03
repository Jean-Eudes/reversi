use crate::domain::board::Board;

pub struct PlayMoveUseCase {}

impl PlayMoveUseCase {
    pub fn execute(&self, board: &mut Board, x: usize, y: usize) {
        board.place(x, y);
    }
}