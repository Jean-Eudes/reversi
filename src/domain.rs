use crate::domain::Case::{Empty, Piece};
use crate::domain::ColorPiece::{Black, White};

#[derive(Copy, Clone, PartialEq)]
pub enum Case {
    Empty,
    Piece(ColorPiece),
}

#[derive(Copy, Clone, PartialEq)]
pub enum ColorPiece {
    White,
    Black,
}

#[derive(PartialEq)]
pub enum PlayerId {
    Player1,
    Player2,
}

pub struct Player(ColorPiece);

impl Player {
    pub fn available_positions(&self, board: &Board) -> Vec<(usize, usize)> {
        let mut available_positions = Vec::new();

        for x in 0..8 {
            for y in 0..8 {
                if let Some(&cell) = board.cell(x, y)
                    && cell != Empty
                {
                    continue;
                }

                for i in -1..=1 {
                    for j in -1..=1 {
                        let pieces = board.scan_flips_in_direction(
                            x,
                            y,
                            i,
                            j,
                            self.opponent_player_color(),
                            self.0,
                        );
                        if pieces.is_some() {
                            available_positions.push((x, y));
                        }
                    }
                }
            }
        }

        available_positions
    }

    fn opponent_player_color(&self) -> ColorPiece {
        match self.0 {
            White => Black,
            Black => White,
        }
    }
}

pub struct Board {
    array: [Case; 64],
    pub current_player: PlayerId,
    player1: Player,
    player2: Player,
}

impl Board {
    pub fn new() -> Board {
        let mut array = [Empty; 64];
        array[27] = Piece(Black);
        array[28] = Piece(White);
        array[35] = Piece(White);
        array[36] = Piece(Black);
        Board {
            array,
            current_player: PlayerId::Player1,
            player1: Player(White),
            player2: Player(Black),
        }
    }

    pub fn current_player(&self) -> &Player {
        match self.current_player {
            PlayerId::Player1 => &self.player1,
            PlayerId::Player2 => &self.player2,
        }
    }

    pub fn place(&mut self, x: usize, y: usize) {
        if let Some(&case) = self.cell(x, y)
            && case == Empty
        {
            let position_available = self.reverse_piece(x, y);
            if position_available {
                self.array[x * 8 + y] = Piece(self.current_player().0);
                self.switch_player();

                if self.current_player().available_positions(self).is_empty() {
                    self.switch_player();
                }
            }
        }
    }

    pub fn reverse_piece(&mut self, x: usize, y: usize) -> bool {
        if let Some(&cell) = self.cell(x, y)
            && cell != Empty
        {
            return false;
        }

        let player = self.current_player().0;
        let opponent = self.current_player().opponent_player_color();
        let mut flipped_any = false;

        // Todo : voir pour utiliser un iterateur par la suite pour mutualiser avec le retournement de pieces.
        for i in -1..=1 {
            for j in -1..=1 {
                let pieces = self.scan_flips_in_direction(x, y, i, j, opponent, player);
                if let Some(pieces) = pieces {
                    flipped_any = true;
                    for piece in pieces {
                        self.array[piece.0 * 8 + piece.1] = Piece(player);
                    }
                }
            }
        }

        flipped_any
    }

    fn scan_flips_in_direction(
        &self,
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
        opponent: ColorPiece,
        player: ColorPiece,
    ) -> Option<Vec<(usize, usize)>> {
        if dx == 0 && dy == 0 {
            return None;
        }

        let mut flips = Vec::new();

        let mut nx = x.checked_add_signed(dx)?;
        let mut ny = y.checked_add_signed(dy)?;

        match self.cell(nx, ny)? {
            Piece(color) if color == &opponent => flips.push((nx, ny)),
            _ => return None,
        }

        loop {
            nx = nx.checked_add_signed(dx)?;
            ny = ny.checked_add_signed(dy)?;

            match self.cell(nx, ny)? {
                Empty => return None,
                Piece(color) if color == &player => return Some(flips),
                Piece(_color) => flips.push((nx, ny)),
            }
        }
    }

    pub fn cell(&self, i: usize, j: usize) -> Option<&Case> {
        if i > 7 || j > 7 {
            return None;
        }
        self.array.get(i * 8 + j)
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            PlayerId::Player1 => PlayerId::Player2,
            PlayerId::Player2 => PlayerId::Player1,
        }
    }

    pub fn end_of_game(&self) -> bool {
        let board_has_cell_empty = self.array.contains(&Empty);
        !board_has_cell_empty
            || (self.player1.available_positions(self).is_empty()
                && self.player2.available_positions(self).is_empty())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_test_if_game_is_over() {
        // Given
        let board = Board::new();

        // When
        let result = board.end_of_game();

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn should_test_position() {
        // Given
        let _board = Board::new();

        // When
        // board.add_black_case(25);

        // Then
        // assert_eq!(board.cell(3, 3), White);
        // assert_eq!(board.cell(3, 4), Black);
        // assert_eq!(board.cell(4, 3), Black);
        // assert_eq!(board.cell(4, 4), Black);
    }
}
