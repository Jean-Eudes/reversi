use macroquad::{
    color::{BLACK, GREEN, WHITE},
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    shapes::{draw_circle, draw_line},
    window::{clear_background, next_frame},
};

use macroquad::prelude::*;
const CELL_SIZE: f32 = 60f32;
const BORDER_SIZE: f32 = 30f32;

use crate::domain::{Board, Case};

mod domain;

#[macroquad::main("Reversi")]
async fn main() {
    println!("Hello, world!");

    let mut plateau = Board::new();
    println!("{}", plateau);
    let white_piece = generate_piece_sprite(40.0, true).await;
    let black_piece = generate_piece_sprite(40.0, false).await;
    loop {
        clear_background(GREEN);
        create_board();
        for i in 0..8 {
            for j in 0..8 {
                if let Some(case2) = plateau.cell(i, j) {
                    match case2 {
                        Case::Empty => {}
                        Case::White => {
                            // draw_texture(
                            //     &white_piece,
                            //     BORDER_SIZE + i as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                            //     BORDER_SIZE + j as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                            //     WHITE,
                            // );

                            draw_piece(
                                BORDER_SIZE + i as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                                BORDER_SIZE + j as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                                20f32,
                                true,
                            )
                        }
                        Case::Black => {
                            draw_piece(
                                BORDER_SIZE + i as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                                BORDER_SIZE + j as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                                20f32,
                                false,
                            );

                            // draw_circle(
                            //     BORDER_SIZE + i as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                            //     BORDER_SIZE + j as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                            //     20f32,
                            //     BLACK,
                            // )
                        }
                    }
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            println!("{mouse_x}, {mouse_y}");

            let x = ((mouse_x - BORDER_SIZE) / CELL_SIZE).floor() as usize;
            let y = ((mouse_y - BORDER_SIZE) / CELL_SIZE).floor() as usize;
            println!("case is {x}, {y}");
            if plateau.available(x, y) {
                plateau.place(x, y);
            }
        }

        next_frame().await
    }
}
fn create_board() {
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

pub async fn generate_piece_sprite(radius: f32, is_white: bool) -> Texture2D {
    let size = (radius * 2.0) as u32;

    // Render target pour dessiner la pièce
    let rt = render_target(size, size);
    set_camera(&Camera2D {
        render_target: Some(rt.clone()),
        ..Default::default()
    });

    clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

    let base = if is_white { WHITE } else { BLACK };

    let cx = radius;
    let cy = radius;

    // Ombre externe douce
    draw_circle(
        cx + 4.0,
        cy + 4.0,
        radius * 0.95,
        Color::new(0.0, 0.0, 0.0, 0.35),
    );

    // Bord sombre
    draw_circle(cx, cy, radius, Color::new(0.05, 0.05, 0.05, 1.0));

    // Couleur principale
    draw_circle(cx, cy, radius * 0.92, base);

    // Bombé simulé (dégradé radial approximé)
    for i in 0..6 {
        let t = i as f32 / 6.0;
        let r = radius * (0.92 - t * 0.15);
        let alpha = 0.08 * (1.0 - t);
        let shade = if is_white {
            Color::new(0.0, 0.0, 0.0, alpha)
        } else {
            Color::new(1.0, 1.0, 1.0, alpha * 0.6)
        };
        draw_circle(cx, cy, r, shade);
    }

    // Highlight elliptique
    draw_ellipse(
        cx - radius * 0.25,
        cy - radius * 0.25,
        radius * 0.55,
        radius * 0.35,
        0.0,
        Color::new(1.0, 1.0, 1.0, if is_white { 0.25 } else { 0.15 }),
    );

    // Ombre interne
    draw_circle(
        cx + radius * 0.15,
        cy + radius * 0.15,
        radius * 0.55,
        Color::new(0.0, 0.0, 0.0, if is_white { 0.15 } else { 0.3 }),
    );

    // Revenir à la caméra normale
    set_default_camera();

    rt.texture
}
