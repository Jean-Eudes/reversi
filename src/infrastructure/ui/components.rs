use macroquad::color::{Color, BLACK, GRAY, WHITE};
use macroquad::prelude::{draw_circle, draw_circle_lines, draw_line};
use crate::{BORDER_SIZE, CELL_SIZE};
use crate::domain::board::{Board, BoardIter, Case};
use crate::domain::board::ColorPiece::White;

pub fn draw_piece(x: f32, y: f32, radius: f32, is_white: bool) {
    // Couleur principale
    let base = if is_white { WHITE } else { BLACK };

    // Ombre portée (léger décalage)
    draw_circle(x + 3.0, y + 3.0, radius, Color::new(0.0, 0.0, 0.0, 0.4));

    // Cercle principal
    draw_circle(x, y, radius, base);

    // Bordure pour donner du relief
    draw_circle_lines(x, y, radius, 3.0, GRAY);

    // Highlight (reflet en haut à gauche)
    draw_circle(
        x - radius * 0.3,
        y - radius * 0.3,
        radius * 0.4,
        Color::new(1.0, 1.0, 1.0, if is_white { 0.25 } else { 0.15 }),
    );

    // Ombre interne (donne un effet bombé)
    draw_circle(
        x + radius * 0.2,
        y + radius * 0.2,
        radius * 0.6,
        Color::new(0.0, 0.0, 0.0, if is_white { 0.15 } else { 0.3 }),
    );
}

pub fn draw_hint(x: f32, y: f32, radius: f32) {
    draw_circle_lines(
        x,
        y,
        radius * 0.8,
        3.0,
        Color::new(0.2, 0.8, 0.2, 0.6), // vert translucide
    );
}

pub fn create_board() {
    for i in 0..=8 {
        draw_line(
            BORDER_SIZE,
            BORDER_SIZE + CELL_SIZE * i as f32,
            BORDER_SIZE + CELL_SIZE * 8f32,
            BORDER_SIZE + CELL_SIZE * i as f32,
            3.0,
            BLACK,
        );
        draw_line(
            BORDER_SIZE + CELL_SIZE * i as f32,
            BORDER_SIZE,
            BORDER_SIZE + CELL_SIZE * i as f32,
            BORDER_SIZE + CELL_SIZE * 8f32,
            3.0,
            BLACK,
        );
    }
}

pub fn create_pieces(plateau: &Board) {
    for (x, y) in BoardIter::new() {
        if let Some(case2) = plateau.cell(x, y) {
            match case2 {
                Case::Empty => {}
                Case::Piece(color) => draw_piece(
                    BORDER_SIZE + x as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                    BORDER_SIZE + y as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                    20f32,
                    *color == White,
                ),
            }
        }
    }
}
