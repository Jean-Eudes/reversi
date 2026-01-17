use macroquad::color::{Color, BLACK, GRAY, WHITE};
use macroquad::prelude::{draw_circle, draw_circle_lines, draw_line, draw_rectangle, draw_rectangle_lines};
use crate::{BORDER_SIZE, CELL_SIZE};
use reversi_core::domain::board::{Board, BoardIter, Case};
use reversi_core::domain::board::ColorPiece::White;

const WOOD_BROWN: Color = Color { r: 0.6, g: 0.4, b: 0.2, a: 1.0 };
const WOOD_DARK: Color = Color { r: 0.4, g: 0.25, b: 0.1, a: 1.0 };

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
    let board_full_size = CELL_SIZE * 8.0;
    
    // Dessin de la bordure "bois" (uniquement l'extérieur)
    let thickness = BORDER_SIZE;

    // Haut
    draw_rectangle(0.0, 0.0, board_full_size + thickness * 2.0, thickness, WOOD_BROWN);
    // Bas
    draw_rectangle(0.0, board_full_size + thickness, board_full_size + thickness * 2.0, thickness, WOOD_BROWN);
    // Gauche
    draw_rectangle(0.0, thickness, thickness, board_full_size, WOOD_BROWN);
    // Droite
    draw_rectangle(board_full_size + thickness, thickness, thickness, board_full_size, WOOD_BROWN);
    
    // Bordures sombres pour donner du relief (cadre extérieur)
    draw_rectangle_lines(
        0.0,
        0.0,
        board_full_size + thickness * 2.0,
        board_full_size + thickness * 2.0,
        5.0,
        WOOD_DARK,
    );
    
    // Ligne interne du cadre
    draw_rectangle_lines(
        BORDER_SIZE,
        BORDER_SIZE,
        board_full_size,
        board_full_size,
        3.0,
        WOOD_DARK,
    );

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
        draw_circle(BORDER_SIZE + CELL_SIZE * 2f32, BORDER_SIZE + CELL_SIZE * 2f32, 5f32, BLACK);
        draw_circle(BORDER_SIZE + CELL_SIZE * 2f32, BORDER_SIZE + CELL_SIZE * 6f32, 5f32, BLACK);
        draw_circle(BORDER_SIZE + CELL_SIZE * 6f32, BORDER_SIZE + CELL_SIZE * 2f32, 5f32, BLACK);
        draw_circle(BORDER_SIZE + CELL_SIZE * 6f32, BORDER_SIZE + CELL_SIZE * 6f32, 5f32, BLACK);
    }
}

pub fn create_pieces(plateau: &Board) {
    for (x, y) in BoardIter::default() {
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
