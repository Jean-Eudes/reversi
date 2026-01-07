use macroquad::{
    color::{GREEN, WHITE},
    input::{is_mouse_button_pressed, mouse_position, MouseButton},
    window::{clear_background, next_frame},
};
use std::iter::repeat_n;

use application::use_case::UseCase;
use macroquad::prelude::*;

const CELL_SIZE: f32 = 60f32;
const BORDER_SIZE: f32 = 30f32;

use crate::domain::board::ColorPiece::{Black, White};
use crate::domain::board::PlayerId::{Player1, Player2};
use crate::domain::board::{Board, ColorPiece};
use crate::infrastructure::ui::components::{create_board, create_pieces, draw_hint, draw_piece};
use crate::infrastructure::ui::fireworks::spawn_firework;

mod application;
mod domain;
mod infrastructure;

enum GameState {
    Start,
    Playing(f64, Board),
    EndGame(EndGameState),
}

enum EndGameState {
    RevealPieces {
        animation_start: f64,
        player1: usize,
        player2: usize,
    },
    Fireworks(f64),
    Lose(f64),
    Draw(f64),
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Reversi".to_owned(),
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let use_case = UseCase::new();

    /*    let _white_piece = generate_piece_sprite(40.0, true).await;
        let _black_piece = generate_piece_sprite(40.0, false).await;

    */
    let reveal_delay = 0.1;

    let mut state = GameState::Start;

    loop {
        match &mut state {
            GameState::Start => {
                let board = use_case.initialize_game_use_case.execute();
                state = GameState::Playing(get_time(), board);
            }
            GameState::Playing(start_time, board) => {
                clear_background(GREEN);
                create_board();
                create_pieces(board);
                
                if let Some(score) = use_case.evaluate_game_end_use_case.execute(board) {
                    state = GameState::EndGame(EndGameState::RevealPieces {
                        animation_start: get_time(),
                        player1: score.player1(),
                        player2: score.player2(),
                    });
                    continue;
                }

                let positions = use_case.compute_available_moves_use_case.execute(board);

                if board.current_player == Player1 {
                    for position in positions {
                        draw_hint(
                            BORDER_SIZE + position.0 as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                            BORDER_SIZE + position.1 as f32 * CELL_SIZE + CELL_SIZE / 2f32,
                            10f32,
                        );
                    }
                }

                if board.current_player == Player1 && is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();

                    let x = ((mouse_x - BORDER_SIZE) / CELL_SIZE).floor() as usize;
                    let y = ((mouse_y - BORDER_SIZE) / CELL_SIZE).floor() as usize;

                    use_case.play_move_use_case.execute(board, x, y);
                    *start_time = get_time();
                } else if board.current_player == Player2 && get_time() - *start_time > 0.8  {
                    use_case.play_ai_move_use_case.execute(board);
                    *start_time = get_time();
                }

            }

            GameState::EndGame(EndGameState::RevealPieces {
                animation_start,
                player1,
                player2,
            }) => {
                clear_background(GREEN);
                create_board();

                let done =
                    create_pieces_for_end_game(*animation_start, reveal_delay, *player1, *player2);
                if done {
                    if player1 > player2 {
                        state = GameState::EndGame(EndGameState::Fireworks(get_time()));
                    } else if player1 < player2 {
                        state = GameState::EndGame(EndGameState::Lose(get_time()));
                    } else if player1 == player2 {
                        state = GameState::EndGame(EndGameState::Draw(get_time()));
                    }
                }
            }
            GameState::EndGame(EndGameState::Fireworks(animation_start)) => {
                launch_fireworks().await;
                state = GameState::Start;
            }
            GameState::EndGame(EndGameState::Draw(animation_start)) => {
                if get_time() - *animation_start < 5.0 {
                    draw_screen();
                } else {
                    state = GameState::Start;
                }
            }
            GameState::EndGame(EndGameState::Lose(animation_start)) => {
                if get_time() - *animation_start < 5.0 {
                    defeat_screen();
                } else {
                    state = GameState::Start;
                }
            }
        }
        next_frame().await
    }
}

fn create_pieces_for_end_game(start_time: f64, delay: f64, player1: usize, player2: usize) -> bool {
    let pieces: Vec<ColorPiece> = repeat_n(Black, player1)
        .chain(repeat_n(White, player2))
        .collect();

    let elapsed = get_time() - start_time;
    let animation_duration = pieces.len() as f64 * delay;

    let count_to_show = (elapsed / delay).floor() as usize;
    let count_to_show = count_to_show.min(pieces.len());

    for (i, color) in pieces.iter().take(count_to_show).enumerate() {
        let x = BORDER_SIZE + (i % 8) as f32 * CELL_SIZE + CELL_SIZE / 2f32;
        let y = BORDER_SIZE + (i / 8) as f32 * CELL_SIZE + CELL_SIZE / 2f32;
        draw_piece(x, y, 20f32, *color == White);
    }

    if elapsed < animation_duration + 1.2 {
        return false;
    }

    true
}

pub async fn launch_fireworks() {
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

        // Nouvelle explosion toutes les 0.7 seconde
        if spawn_timer > 0.7 {
            spawn_firework(&mut particles);
            spawn_timer = 0.0;
        }

        // Mise à jour des particules
        particles.iter_mut().for_each(|p| p.update(dt));
        particles.retain(|p| p.life() > 0.0);

        // Dessin
        // clear_background(BLACK);

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

pub fn defeat_screen() {
    draw_text(
        "You Lost !",
        screen_width() / 2.0 - 120.0,
        80.0,
        60.0,
        WHITE,
    );
}

pub fn draw_screen() {
    draw_text(
        "You Lost !",
        screen_width() / 2.0 - 120.0,
        80.0,
        60.0,
        WHITE,
    );
}
