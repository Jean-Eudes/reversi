use crate::application::move_use_case::MoveUseCase;
use crate::domain::board::Board;

pub struct AIMoveUseCase {
    move_use_case: Box<dyn MoveUseCase>,
}

pub struct SelectedMove {
    position: (usize, usize),
    pieces_to_flip: Vec<(usize, usize)>,
}

impl SelectedMove {
    pub fn new(position: (usize, usize), pieces_to_flip: Vec<(usize, usize)>) -> Self {
        Self {
            position,
            pieces_to_flip,
        }
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn pieces_to_flip(self) -> Vec<(usize, usize)> {
        self.pieces_to_flip
    }
}

impl AIMoveUseCase {
    pub fn new(move_use_case: Box<dyn MoveUseCase>) -> Self {
        Self { move_use_case }
    }

    pub fn execute(&self, board: &mut Board) -> Option<SelectedMove> {
        let available_moves = board.available_positions(board.current_player());
        if available_moves.is_empty() {
            return None;
        }

        let num = fastrand::usize(0..available_moves.len());
        let position_choose = available_moves[num];
        let move_result = self
            .move_use_case
            .execute(board, position_choose.0, position_choose.1);
        move_result.map(|moves| SelectedMove {
            position: position_choose,
            pieces_to_flip: moves,
        })
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
