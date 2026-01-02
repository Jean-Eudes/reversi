use macroquad::{
    color::{BLACK, GREEN, WHITE},
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    shapes::{draw_circle, draw_line},
    window::{clear_background, next_frame},
};

use macroquad::prelude::*;
const CELL_SIZE: f32 = 60f32;
const BORDER_SIZE: f32 = 30f32;

use crate::domain::board::ColorPiece::White;
use crate::domain::board::PlayerId::{Player1, Player2};
use crate::domain::board::{Board, BoardIter, Case};

mod domain;

fn window_conf() -> Conf {
    Conf {
        window_title: "Reversi".to_owned(),
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut plateau = Board::new();
    let _white_piece = generate_piece_sprite(40.0, true).await;
    let _black_piece = generate_piece_sprite(40.0, false).await;
    loop {
        if !plateau.end_of_game() {
            clear_background(GREEN);
            create_board();
            create_pieces(&plateau);
            let positions = plateau.available_positions(plateau.current_player());
            for position in positions {
                draw_hint(
                    BORDER_SIZE + position.0 as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                    BORDER_SIZE + position.1 as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                    10f32,
                );
            }

            if plateau.current_player == Player1 && is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();

                let x = ((mouse_x - BORDER_SIZE) / CELL_SIZE).floor() as usize;
                let y = ((mouse_y - BORDER_SIZE) / CELL_SIZE).floor() as usize;
                plateau.place(x, y);
            } else if plateau.current_player == Player2 {
                let vec = plateau.available_positions(plateau.current_player());
                let num = rand::gen_range(0, vec.len());
                plateau.place(vec[num].0, vec[num].1);
            }
        } else {
            victory_fireworks().await;
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

fn create_pieces(plateau: &Board) {
    for (x, y) in BoardIter::new() {
        if let Some(case2) = plateau.cell(x, y) {
            match case2 {
                Case::Empty => {}
                Case::Piece(color) => {
                    // draw_texture(
                    //     &white_piece,
                    //     BORDER_SIZE + i as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                    //     BORDER_SIZE + j as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                    //     WHITE,
                    // );

                    draw_piece(
                        BORDER_SIZE + x as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                        BORDER_SIZE + y as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                        20f32,
                        *color == White,
                    )
                }
            }
        }
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

pub fn draw_hint(x: f32, y: f32, radius: f32) {
    draw_circle_lines(
        x,
        y,
        radius * 0.8,
        3.0,
        Color::new(0.2, 0.8, 0.2, 0.6), // vert translucide
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
struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Color,
    life: f32,
}

impl Particle {
    fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.vel *= 0.98; // friction légère
        self.life -= dt;
    }

    fn draw(&self) {
        let alpha = self.life.clamp(0.0, 1.0);
        let mut c = self.color;
        c.a = alpha;
        draw_circle(self.pos.x, self.pos.y, 3.0, c);
    }
}

fn spawn_firework(particles: &mut Vec<Particle>) {
    let center = vec2(
        rand::gen_range(100.0, screen_width() - 100.0),
        rand::gen_range(100.0, screen_height() - 200.0),
    );

    let base_color = Color::new(
        rand::gen_range(0.5, 1.0),
        rand::gen_range(0.5, 1.0),
        rand::gen_range(0.5, 1.0),
        1.0,
    );

    for _ in 0..40 {
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let speed = rand::gen_range(80.0, 200.0);

        particles.push(Particle {
            pos: center,
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: base_color,
            life: rand::gen_range(0.8, 1.5),
        });
    }
}

pub async fn victory_fireworks() {
    let mut particles = Vec::new();
    let mut timer = 0.0;
    let mut spawn_timer = 0.0;

    loop {
        let dt = get_frame_time();
        timer += dt;
        spawn_timer += dt;

        // Arrêter après 15 secondes
        if timer > 15.0 {
            break;
        }

        // Nouvelle explosion toutes les 0.7 secondes
        if spawn_timer > 0.7 {
            spawn_firework(&mut particles);
            spawn_timer = 0.0;
        }

        // Mise à jour des particules
        particles.iter_mut().for_each(|p| p.update(dt));
        particles.retain(|p| p.life > 0.0);

        // Dessin
        clear_background(BLACK);

        for p in &particles {
            p.draw();
        }

        draw_text(
            "Victoire !",
            screen_width() / 2.0 - 120.0,
            80.0,
            60.0,
            WHITE,
        );

        next_frame().await;
    }
}
