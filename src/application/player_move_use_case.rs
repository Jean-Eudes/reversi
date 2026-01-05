use crate::application::move_use_case::MoveUseCase;
use crate::domain::board::Board;

pub struct PlayerMoveUseCase {
    move_use_case: Box<dyn MoveUseCase>,
}

impl PlayerMoveUseCase {
    pub fn new(move_use_case: Box<dyn MoveUseCase>) -> Self {
        Self { move_use_case }
    }

    pub fn execute(&self, board: &mut Board, x: usize, y: usize) {
        self.move_use_case.execute(board, x, y);
    }
}
