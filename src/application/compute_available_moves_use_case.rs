use crate::domain::board::Board;

pub struct ComputeAvailableMovesUseCase {}

impl ComputeAvailableMovesUseCase {
    pub fn execute(&self, board: &Board) -> Vec<(usize, usize)> {
        let player = board.current_player();
        board.available_positions(player)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compute_available_moves_for_initial_board() {
        // Given
        let use_case = ComputeAvailableMovesUseCase {};
        let board = Board::new();

        // When
        let result = use_case.execute(&board);

        // Then
        assert_eq!(result, vec![(3, 2), (2, 3), (5, 4), (4, 5)]);
    }
}
