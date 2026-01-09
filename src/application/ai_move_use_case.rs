use crate::application::move_use_case::MoveUseCase;
use crate::domain::board::Board;
use macroquad::prelude::rand;

pub struct AIMoveUseCase {
    move_use_case: Box<dyn MoveUseCase>,
}

impl AIMoveUseCase {
    pub fn new(move_use_case: Box<dyn MoveUseCase>) -> Self {
        Self { move_use_case }
    }

    pub fn execute(&self, board: &mut Board) -> Option<Vec<(usize, usize)>> {
        let available_moves = board.available_positions(board.current_player());
        if available_moves.is_empty() {
            return None;
        }
        let num = rand::gen_range(0, available_moves.len());
        self.move_use_case
            .execute(board, available_moves[num].0, available_moves[num].1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::move_use_case::MockMoveUseCase;
    use crate::domain::board::Case;
    use crate::domain::board::Case::Empty;
    use crate::domain::board::ColorPiece::{Black, White};
    use mockall::predicate;

    #[test]
    fn should_play_random_move() {
        // Given
        let mut array = [Empty; 64];
        array[1] = Case::Piece(Black);
        array[2] = Case::Piece(White);
        let mut board = Board::create_board_for_test(array);
        let mut move_use_case_mock = MockMoveUseCase::new();
        move_use_case_mock
            .expect_execute()
            .with(predicate::always(), predicate::eq(0), predicate::eq(3))
            .return_const(vec![]);

        let ai_move_use_case = AIMoveUseCase::new(Box::new(move_use_case_mock));

        // When / Then
        ai_move_use_case.execute(&mut board);
    }
}
