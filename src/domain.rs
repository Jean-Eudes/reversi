use std::fmt::{Display, Formatter};

use crate::domain::Case::{Black, Empty, White};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Case {
    Empty,
    White,
    Black,
}

#[derive(Debug)]
enum Player {
    White,
    Black,
}

pub struct Board {
    array: [Case; 64],
    current_player: Player,
}

impl Board {
    pub fn new() -> Board {
        let mut array = [Empty; 64];
        array[27] = Black;
        array[28] = White;
        array[35] = White;
        array[36] = Black;
        Board {
            array,
            current_player: Player::White,
        }
    }

    pub fn place(&mut self, x: usize, y: usize) {
        match self.current_player {
            Player::White => {
                if let Some(&case) = self.cell(x, y)
                    && case == Empty
                {
                    self.array[x * 8 + y] = White;
                }
                self.current_player = Player::Black;
            }
            Player::Black => {
                if let Some(&case) = self.cell(x, y)
                    && case == Empty
                {
                    self.array[x * 8 + y] = Black;
                }
                self.current_player = Player::White;
            }
        }
    }

    pub fn available(&self, x: usize, y: usize) -> bool {
        if let Some(&cell) = self.cell(x, y)
            && cell != Empty
        {
            return true;
        }

        // Todo : voirpour utilisier un iterateur par la suite pour mutualiser avec le retournement de pieces.
        for i in -1..=1 {
            for j in -1..=1 {
                if let (Some(nx), Some(ny)) = (x.checked_add_signed(i), y.checked_add_signed(j))
                    && !(i == 0 && j == 0)
                {
                    match self.current_player {
                        Player::White => {
                            if let Some(&cell) = self.cell(nx, ny)
                                && cell == Black
                            {
                                for k in 2..=8 {
                                    if let (Some(nx), Some(ny)) =
                                        (x.checked_add_signed(k * i), y.checked_add_signed(k * j))
                                    {
                                        let cell = self.cell(nx, ny);
                                        if cell.filter(|&&c| c == White).is_some() {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                        Player::Black => {
                            if let Some(&cell) = self.cell(nx, ny)
                                && cell == White
                            {
                                for k in 2..=8 {
                                    if let (Some(nx), Some(ny)) =
                                        (x.checked_add_signed(k * i), y.checked_add_signed(k * j))
                                    {
                                        let cell = self.cell(nx, ny);
                                        if cell.filter(|&&c| c == Black).is_some() {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                println!("{i} {j}");
            }
        }

        false
    }

    pub fn cell(&self, i: usize, j: usize) -> Option<&Case> {
        self.array.get(i * 8 + j)
    }

    pub fn end_of_game(&self) -> bool {
        false
    }
}

impl Case {
    fn to_char(&self) -> char {
        match self {
            Empty => ' ',
            White => 'W',
            Black => 'B',
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut display = String::from("");
        for i in 0..8 {
            for j in 0..8 {
                display.push(self.array[8 * i + j].to_char())
            }
            display.push('\n')
        }
        write!(f, "{}", display)
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
    fn should_test_if_a_case_is_occupied() {
        // Given
        let board = Board::new();

        // When
        let result = board.available(2, 2);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn should_test_if_a_case_is_free() {
        // Given
        let board = Board::new();

        // When
        let result = board.available(3, 3);

        // Then
        assert_eq!(result, true);
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
