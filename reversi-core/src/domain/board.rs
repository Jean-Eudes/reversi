use crate::domain::board::Case::{Empty, Piece};
use crate::domain::board::ColorPiece::{Black, White};
use crate::domain::directions::Directions;
use crate::domain::player::Player;

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Case {
    Empty,
    Piece(ColorPiece),
}

impl Case {
    fn flip(&mut self) {
        *self = match self {
            Empty => Empty,
            Piece(White) => Piece(Black),
            Piece(Black) => Piece(White),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum ColorPiece {
    White,
    Black,
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum PlayerId {
    Player1,
    Player2,
}

pub struct Board {
    array: [Case; 64],
    current_player: PlayerId,
    player1: Player,
    player2: Player,
}

#[cfg_attr(test, derive(Debug))]
pub struct Score {
    player1: usize,
    player2: usize,
}

impl Score {
    pub fn player1(&self) -> usize {
        self.player1
    }
    pub fn player2(&self) -> usize {
        self.player2
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut array = [Empty; 64];
        array[27] = Piece(White);
        array[28] = Piece(Black);
        array[35] = Piece(Black);
        array[36] = Piece(White);
        Board {
            array,
            current_player: PlayerId::Player1,
            player1: Player::new(Black),
            player2: Player::new(White),
        }
    }
}

impl Board {
    #[cfg(test)]
    pub fn create_board_for_test(array: [Case; 64]) -> Board {
        Board {
            array,
            current_player: PlayerId::Player1,
            player1: Player::new(Black),
            player2: Player::new(White),
        }
    }

    pub fn current_player(&self) -> &Player {
        match self.current_player {
            PlayerId::Player1 => &self.player1,
            PlayerId::Player2 => &self.player2,
        }
    }

    pub fn player1(&self) -> bool {
        self.current_player == PlayerId::Player1
    }

    pub fn player2(&self) -> bool {
        self.current_player == PlayerId::Player2
    }

    pub fn available_positions(&self, player: &Player) -> Vec<(usize, usize)> {
        let mut available_positions = Vec::new();

        for (x, y) in BoardIter::default() {
            if self.cell(x, y) != Some(&Empty) {
                continue;
            }
            let directions = Directions::default();
            for (dx, dy) in directions {
                let pieces = self.scan_flips_in_direction(
                    x,
                    y,
                    dx,
                    dy,
                    player.opponent_color(),
                    player.color(),
                );
                if pieces.is_some() {
                    available_positions.push((x, y));
                    break;
                }
            }
        }
        available_positions
    }

    pub fn place(&mut self, x: usize, y: usize) -> Option<Vec<(usize, usize)>> {
        if !(0..8).contains(&x) || !(0..8).contains(&y) {
            return None;
        }
        let position_available = self.flip(x, y)?;
        self.array[x * 8 + y] = Piece(self.current_player().color());
        self.switch_player();

        if self.available_positions(self.current_player()).is_empty() {
            self.switch_player();
        }
        Some(position_available)
    }

    fn flip(&mut self, x: usize, y: usize) -> Option<Vec<(usize, usize)>> {
        if matches!(self.cell(x, y), Some(Piece(_))) {
            return None;
        }

        let player = self.current_player().color();
        let opponent = self.current_player().opponent_color();
        let mut flipped_pieces = Vec::new();

        let directions = Directions::default();
        for (dx, dy) in directions {
            let pieces = self.scan_flips_in_direction(x, y, dx, dy, opponent, player);
            if let Some(mut pieces) = pieces {
                for piece in &pieces {
                    self.array[piece.0 * 8 + piece.1].flip();
                }
                flipped_pieces.append(&mut pieces);
            };
        }
        if flipped_pieces.is_empty() {
            None
        } else {
            Some(flipped_pieces)
        }
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

    pub fn end_of_game(&self) -> Option<Score> {
        let board_has_cell_empty = self.array.contains(&Empty);
        if !board_has_cell_empty
            || (self.available_positions(&self.player1).is_empty()
                && self.available_positions(&self.player2).is_empty())
        {
            Some(Score {
                player1: self
                    .array
                    .iter()
                    .filter(|&&c| c == Piece(self.player1.color()))
                    .count(),
                player2: self
                    .array
                    .iter()
                    .filter(|&&c| c == Piece(self.player2.color()))
                    .count(),
            })
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct BoardIter {
    x: usize,
    y: usize,
}

impl Iterator for BoardIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= 8 {
            return None;
        }

        let pos = (self.x, self.y);

        self.x += 1;
        if self.x >= 8 {
            self.x = 0;
            self.y += 1;
        }

        Some(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_initialize_board_with_correct_initial_state_and_player() {
        // Given / When
        let board = Board::default();

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
        BoardIter::default()
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

    #[test]
    fn should_game_is_ending_when_board_is_full_of_white_pieces() {
        // Given
        let board = Board::create_board_for_test([Piece(White); 64]);

        // When
        let result = board.end_of_game();

        // Then
        assert!(result.is_some());
        let score = result.expect("Score must be Some");
        assert_eq!(score.player1(), 0);
        assert_eq!(score.player2(), 64);
    }

    #[test]
    fn should_game_is_ending_when_board_is_full_of_black_pieces() {
        // Given
        let board = Board::create_board_for_test([Piece(Black); 64]);

        // When
        let result = board.end_of_game();

        // Then
        assert!(result.is_some());
        let score = result.expect("Score must be Some");
        assert_eq!(score.player1(), 64);
        assert_eq!(score.player2(), 0);
    }

    #[test]
    fn should_game_is_ending_when_no_move_is_available() {
        // Given
        let mut array = [Empty; 64];
        array[3 * 8 + 3] = Piece(Black);
        array[3 * 8 + 4] = Piece(Black);
        let board = Board::create_board_for_test(array);

        // When
        let result = board.end_of_game();

        // Then
        assert!(result.is_some());
        let score = result.expect("Score must be Some");
        assert_eq!(score.player1(), 2);
        assert_eq!(score.player2(), 0);
    }

    #[test]
    fn should_game_is_not_ending_when_game_is_started() {
        // Given
        let board = Board::default();

        // When
        let result = board.end_of_game();

        // Then
        assert!(result.is_none());
    }

    #[test]
    fn should_flip_white_piece_to_black() {
        // Given
        let mut case = Piece(White);

        // When
        case.flip();

        // Then
        assert_eq!(case, Piece(Black))
    }

    #[test]
    fn should_flip_black_piece_to_white() {
        // Given
        let mut case = Piece(Black);

        // When
        case.flip();

        // Then
        assert_eq!(case, Piece(White))
    }

    #[test]
    fn should_switch_player() {
        // Given
        let mut board = Board::default();

        // When
        board.switch_player();

        // Then
        assert_eq!(board.current_player, PlayerId::Player2);
    }

    #[test]
    fn should_compute_available_moves_for_initial_board() {
        // Given
        let board = Board::default();

        // When
        let result = board.available_positions(board.current_player());

        // Then
        assert_eq!(result, vec![(3, 2), (2, 3), (5, 4), (4, 5)]);
    }

    #[test]
    fn should_not_flip_when_cell_is_already_occupied() {
        // Given
        let mut board = Board::default();
        let x = 3;
        let y = 3;

        // When
        let result = board.flip(x, y);

        // Then
        assert!(result.is_none());
    }

    #[test]
    fn should_not_flip_when_cell_is_empty_but_not_flippable() {
        // Given
        let mut board = Board::default();
        let x = 0;
        let y = 0;

        // When
        let result = board.flip(x, y);

        // Then
        assert!(result.is_none());
    }

    #[test]
    fn should_flip_when_cell_is_empty_and_flippable() {
        // Given
        let mut board = Board::default();
        let x = 3;
        let y = 2;

        // When
        let result = board.flip(x, y);

        // Then
        assert!(result.is_some());
        let flipped_pieces = result.expect("Should have flipped pieces");
        assert_eq!(flipped_pieces, vec![(3, 3)]);
        assert_eq!(board.cell(3, 3), Some(&Piece(Black)));
    }

    #[test]
    fn should_not_place_when_cell_is_already_occupied() {
        // Given
        let mut board = Board::default();
        let x = 3;
        let y = 3;

        // When
        let result = board.place(x, y);

        // Then
        assert!(result.is_none());
    }

    #[test]
    fn should_not_place_when_cell_is_empty_but_not_flippable() {
        // Given
        let mut board = Board::default();
        let x = 0;
        let y = 0;

        // When
        let result = board.place(x, y);

        // Then
        assert!(result.is_none());
    }

    #[test]
    fn should_place_when_cell_is_empty_and_flippable() {
        // Given
        let mut board = Board::default();
        let x = 3;
        let y = 2;

        // When
        let result = board.place(x, y);

        // Then
        assert!(result.is_some());
        let flipped_pieces = result.expect("Should have flipped pieces");
        assert_eq!(flipped_pieces, vec![(3, 3)]);
        assert_eq!(board.cell(3, 3), Some(&Piece(Black)));
        assert_eq!(board.cell(3, 2), Some(&Piece(Black)));
        assert_eq!(board.current_player, PlayerId::Player2);
    }

    #[test]
    fn should_return_none_when_place_is_out_of_bounds() {
        // Given
        let mut board = Board::default();
        let x = 8;
        let y = 0;

        // When
        let result = board.place(x, y);

        // Then
        assert!(result.is_none());
    }
}
