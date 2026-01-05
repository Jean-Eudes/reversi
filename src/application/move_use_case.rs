use crate::domain::board::Board;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait MoveUseCase {
    fn execute(&self, board: &mut Board, x: usize, y: usize);
}

pub struct MoveUseCaseImpl {}

impl MoveUseCase for MoveUseCaseImpl {
    fn execute(&self, board: &mut Board, x: usize, y: usize) {
        board.place(x, y);
    }
}
