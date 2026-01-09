use crate::domain::board::Board;

pub struct ComputeAvailableMovesUseCase {}

impl ComputeAvailableMovesUseCase {
    pub fn execute(&self, board: &Board) -> Vec<(usize, usize)> {
        let player = board.current_player();
        board.available_positions(player)
    }
}
