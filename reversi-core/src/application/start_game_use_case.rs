use crate::domain::board::Board;

pub struct StartGameUseCase {}

impl StartGameUseCase {
    pub fn execute(&self) -> Board {
        Board::default()
    }
}
