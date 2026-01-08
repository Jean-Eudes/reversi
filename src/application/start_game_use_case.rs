use crate::domain::board::Board;

pub struct StartGameUseCase {}

impl StartGameUseCase {
    pub fn execute(&self) -> Board {
        Board::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::board::Case::{Empty, Piece};
    use crate::domain::board::ColorPiece::{Black, White};
    use crate::domain::board::{BoardIter, PlayerId};

    #[test]
    fn should_initialize_game_with_correct_initial_state_and_player() {
        // Given
        let use_case = StartGameUseCase {};

        // When
        let board = use_case.execute();

        // Then
        assert_eq!(
            board.current_player,
            PlayerId::Player1,
            "Initial player should be player1"
        );
        assert_eq!(board.cell(3, 3), Some(&Piece(White)));
        assert_eq!(board.cell(3, 4), Some(&Piece(Black)));
        assert_eq!(board.cell(4, 3), Some(&Piece(Black)));
        assert_eq!(board.cell(4, 4), Some(&Piece(White)));
        BoardIter::new()
            .filter(|(x, y)| !matches!((x, y), (3 | 4, 3 | 4)))
            .for_each(|(x, y)| {
                assert_eq!(
                    board.cell(x, y),
                    Some(&Empty),
                    "Cell ({}, {}) should be empty",
                    x,
                    y
                )
            });
    }
}
