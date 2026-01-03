use macroquad::prelude::rand;
use crate::domain::board::Board;

pub struct PlayAIMoveUseCase {}

impl PlayAIMoveUseCase {
    pub fn execute(&self, board: &mut Board) {
        let available_moves = board.available_positions(board.current_player());
        let num = rand::gen_range(0, available_moves.len());
        board.place(available_moves[num].0, available_moves[num].1);
    }
}