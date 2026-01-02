use crate::domain::board::ColorPiece;
use crate::domain::board::ColorPiece::{Black, White};

pub struct Player(ColorPiece);

impl Player {
    pub fn new(color: ColorPiece) -> Player {
        Player(color)
    }
    pub fn opponent_color(&self) -> ColorPiece {
        match self.0 {
            White => Black,
            Black => White,
        }
    }

    pub fn color(&self) -> ColorPiece {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_return_colors_for_white_player() {
        // Given
        let player = Player::new(White);

        // When / then
        assert_eq!(player.color(), White);
        assert_eq!(player.opponent_color(), Black);
    }
    
    #[test]
    pub fn should_return_colors_for_black_player() {
        // Given
        let player = Player::new(Black);

        // When / then
        assert_eq!(player.color(), Black);
        assert_eq!(player.opponent_color(), White);
    }
}