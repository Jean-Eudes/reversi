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

enum Player {
    Player1,
    Player2,
}

pub struct Board {
    array: [Case; 64],
    current_player: Player,
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
            current_player: Player::Player1,
        }
    }

    pub fn place(&mut self, x: usize, y: usize) {
        match self.current_player {
            Player::Player1 => {
                if let Some(&case) = self.cell(x, y)
                    && case == Empty
                {
                    let x1 = self.reverse_piece(x, y);
                    if x1 {
                        self.array[x * 8 + y] = Piece(White);
                    }
                }
                self.current_player = Player::Player2;
            }
            Player::Player2 => {
                if let Some(&case) = self.cell(x, y)
                    && case == Empty
                {
                    let x2 = self.reverse_piece(x, y);
                    if x2 {
                        self.array[x * 8 + y] = Piece(Black);
                    }
                }
                self.current_player = Player::Player1;
            }
        }
    }
    pub fn reverse_piece(&mut self, x: usize, y: usize) -> bool {
        if let Some(&cell) = self.cell(x, y)
            && cell != Empty
        {
            return false;
        }

        let mut must_return_piece = false;

        // Todo : voir pour utiliser un iterateur par la suite pour mutualiser avec le retournement de pieces.
        for i in -1..=1 {
            for j in -1..=1 {
                match self.current_player {
                    Player::Player1 => {
                        must_return_piece |= self.reverse_line(x, y, i, j, Black, White);
                    }
                    Player::Player2 => {
                        must_return_piece |= self.reverse_line(x, y, i, j, White, Black);
                    }
                }
            }
        }

        must_return_piece
    }

    fn reverse_line(
        &mut self,
        x: usize,
        y: usize,
        i: isize,
        j: isize,
        color_opposite_player: ColorPiece,
        color_player: ColorPiece,
    ) -> bool {
        let mut must_return_piece = false;

        if let (Some(nx), Some(ny)) = (x.checked_add_signed(i), y.checked_add_signed(j))
            && !(i == 0 && j == 0)
        {
            let mut pieces = vec![];
            if let Some(&cell) = self.cell(nx, ny)
                && let Piece(color) = cell
                && color == color_opposite_player
            {
                pieces.push((nx, ny));
                for k in 2..=8 {
                    if let (Some(nx), Some(ny)) =
                        (x.checked_add_signed(k * i), y.checked_add_signed(k * j))
                    {
                        let cell = self.cell(nx, ny);
                        if cell.filter(|&&c| c == Empty).is_some() {
                            continue;
                        }
                        if let Some(case) = cell
                            && let Piece(color) = case
                            && color == &color_player
                        {
                            //cell.filter(|&&c| c == color_player).is_some() {
                            pieces.iter().for_each(|(a, b)| {
                                self.array[a * 8 + b] = Piece(color_player);
                            });
                            must_return_piece |= true;
                            continue;
                        } else {
                            pieces.push((nx, ny));
                        }
                    }
                }
            }
        }
        must_return_piece
    }

    pub fn cell(&self, i: usize, j: usize) -> Option<&Case> {
        if i > 7 || j > 7 {
            return None;
        }
        self.array.get(i * 8 + j)
    }

    pub fn end_of_game(&self) -> bool {
        false
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
        let board = Board::new();

        // When
        // board.add_black_case(25);

        // Then
        // assert_eq!(board.cell(3, 3), White);
        // assert_eq!(board.cell(3, 4), Black);
        // assert_eq!(board.cell(4, 3), Black);
        // assert_eq!(board.cell(4, 4), Black);
    }
}
