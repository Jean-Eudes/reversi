use crate::domain::board::{Board, Score};

pub struct EvaluateGameEndUseCase {}

impl EvaluateGameEndUseCase {
    pub fn execute(&self, board: &Board) -> Option<Score> {
        board.end_of_game()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::board::Case;
    use crate::domain::board::Case::Empty;
    use crate::domain::board::ColorPiece::{Black, White};

    #[test]
    fn should_game_is_ending_when_board_is_full_of_white_pieces() {
        // Given
        let use_case = EvaluateGameEndUseCase {};
        let board = Board::create_board_for_test([Case::Piece(White); 64]);

        // When
        let result = use_case.execute(&board);

        // Then
        assert!(matches!(result, Some(_)));
        let score = result.unwrap();
        assert_eq!(score.player1(), 64);
        assert_eq!(score.player2(), 0);
    }
    #[test]
    fn should_game_is_ending_when_board_is_full_of_black_pieces() {
        // Given
        let use_case = EvaluateGameEndUseCase {};
        let board = Board::create_board_for_test([Case::Piece(Black); 64]);

        // When
        let result = use_case.execute(&board);

        // Then
        assert!(matches!(result, Some(_)));
        let score = result.unwrap();
        assert_eq!(score.player1(), 0);
        assert_eq!(score.player2(), 64);
    }

    #[test]
    fn should_game_is_ending_when_no_move_is_available() {
        // Given
        let use_case = EvaluateGameEndUseCase {};
        let mut array = [Empty; 64];
        array[3 * 8 + 3] = Case::Piece(White);
        array[3 * 8 + 4] = Case::Piece(White);
        let board = Board::create_board_for_test(array);

        // When
        let result = use_case.execute(&board);

        // Then
        assert!(matches!(result, Some(_)));
        let score = result.unwrap();
        assert_eq!(score.player1(), 2);
        assert_eq!(score.player2(), 0);
    }

}
