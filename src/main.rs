use crate::{resources::rectangle::Player, utils::helpers::parse_digit};
use macroquad::prelude::*;
use resources::ball::Ball;
use utils::{
    ball_lost::detect_loss,
    collision::detect_collision,
    helpers::{reset_game, restart_game},
    text_format::{draw_description_text, draw_error_text, draw_header_text, draw_text},
};

use macroquad_particles::{self as particles, AtlasConfig, BlendMode, Emitter, EmitterConfig};

mod resources;
mod utils;

const DESCRIPTION_FONT_SIZE: u16 = 32;
const PLAYER_SCORE_SIZE: u16 = 35;

enum GameState {
    Menu(MenuState),
    Game,
    Pause,
    Complete,
}
#[derive(Clone, Copy)]
enum MenuState {
    Landing,
    Rounds,
    Tutorial,
    Difficulty,
}

fn fire() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 0.8,
        lifetime_randomness: 0.1,
        emitting: false,
        amount: 12,
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        initial_direction_spread: 0.5,
        size: 15.0,
        blend_mode: BlendMode::Additive,
        ..Default::default()
    }
}

#[macroquad::main("Takedown")]
async fn main() {
    let font = load_ttf_font("./font/Anton-Regular.ttf").await.unwrap();

    let mut game_state = GameState::Menu(MenuState::Landing);

    let mut player_1_score = 0;
    let mut player_2_score = 0;

    let mut player_1 = Player::new(10.0);
    let mut player_2 = Player::new(screen_width() - 20.0);

    let mut round = 0;
    let mut level = 0.0;
    let mut difficulty = 0;

    let mut ball = Ball::new(vec2(screen_height() * 0.5, screen_width() * 0.5));

    let mut invalid_input = false;
    let mut invalid_difficulty = false;

    let texture = load_texture("./src/images/smoke_fire.png").await.unwrap();
    let mut flying_emitter_world = Emitter::new(EmitterConfig {
        local_coords: false,
        texture: Some(texture),
        ..fire()
    });

    loop {
        match game_state {
            GameState::Menu(state) => {
                draw_header_text(&format!("TakeDown"), font);
                match state {
                    MenuState::Landing => {
                        if is_key_pressed(KeyCode::Space) {
                            game_state = GameState::Menu(MenuState::Rounds);
                        }
                        if is_key_pressed(KeyCode::T) {
                            game_state = GameState::Menu(MenuState::Tutorial);
                        }

                        draw_description_text(
                            &format!("Press T to open tutorial"),
                            font,
                            screen_height() * 0.40,
                            25,
                        );

                        draw_description_text(
                            &format!("Press Space to start the game"),
                            font,
                            screen_height() * 0.55,
                            DESCRIPTION_FONT_SIZE,
                        );
                    }
                    MenuState::Rounds => {
                        // Check for key presses and update the level accordingly
                        if let Some(key) = get_last_key_pressed() {
                            if let Some(digit) = parse_digit(key) {
                                round = digit;
                            }
                            invalid_input = true;
                        }

                        draw_description_text(
                            &format!("Select a Score limit from 1 - 9"),
                            font,
                            screen_height() * 0.40,
                            DESCRIPTION_FONT_SIZE,
                        );

                        // If the user enters an invalid key, prompt them to enter a valid number
                        if invalid_input {
                            draw_error_text(font);
                        }
                        if round > 0 {
                            game_state = GameState::Menu(MenuState::Difficulty);
                        }
                    }
                    MenuState::Difficulty => {
                        invalid_input = false;
                        if is_key_pressed(KeyCode::Escape) {
                            round = 0;
                            game_state = GameState::Menu(MenuState::Rounds)
                        }

                        if is_key_pressed(KeyCode::Key1) {
                            level = 300.0;
                            difficulty = 1;
                        } else if is_key_pressed(KeyCode::Key2) {
                            level = 350.0;
                            difficulty = 2;
                        } else if is_key_pressed(KeyCode::Key3) {
                            level = 400.0;
                            difficulty = 3;
                        } else if is_key_pressed(KeyCode::Key4) {
                            level = 500.0;
                            difficulty = 4;
                        } else if is_key_pressed(KeyCode::Key5) {
                            level = 550.0;
                            difficulty = 5;
                        } else if let Some(key) = get_last_key_pressed() {
                            if let Some(digit) = parse_digit(key) {
                                difficulty = digit;
                            } else {
                                invalid_difficulty = true; // Set the flag to display the message
                            }
                        }

                        draw_description_text(
                            &format!("Select a difficulty level from 1 - 5"),
                            font,
                            screen_height() * 0.40,
                            DESCRIPTION_FONT_SIZE,
                        );
                        draw_description_text(
                            &format!("Press ESC to go Back"),
                            font,
                            screen_height() * 0.98,
                            20,
                        );
                        if invalid_difficulty {
                            draw_error_text(font);
                        }
                        if level > 0.0 {
                            game_state = GameState::Game;
                        }
                    }
                    MenuState::Tutorial => {
                        draw_description_text(
                            &format!("Tutorial"),
                            font,
                            120.0,
                            DESCRIPTION_FONT_SIZE,
                        );
                        draw_text(
                            &format!("P1"),
                            font,
                            screen_width() * 0.12,
                            screen_height() * 0.3,
                            28,
                            WHITE,
                        );
                        draw_text(
                            &format!("W - UP"),
                            font,
                            screen_width() * 0.12,
                            screen_height() * 0.45,
                            25,
                            WHITE,
                        );
                        draw_text(
                            &format!("S - DOWN"),
                            font,
                            screen_width() * 0.12,
                            screen_height() * 0.55,
                            25,
                            WHITE,
                        );

                        draw_text(
                            &format!("P2"),
                            font,
                            screen_width() * 0.88,
                            screen_height() * 0.3,
                            28,
                            WHITE,
                        );
                        draw_text(
                            &format!("Key - UP"),
                            font,
                            screen_width() * 0.88,
                            screen_height() * 0.45,
                            25,
                            WHITE,
                        );
                        draw_text(
                            &format!("Key - DOWN"),
                            font,
                            screen_width() * 0.88,
                            screen_height() * 0.55,
                            25,
                            WHITE,
                        );
                        draw_description_text(
                            &format!("Touching the Yellow provides Boost!"),
                            font,
                            screen_height() * 0.5,
                            25,
                        );
                        draw_description_text(
                            &format!("Press space to continue"),
                            font,
                            screen_height() * 0.98,
                            DESCRIPTION_FONT_SIZE,
                        );
                        if is_key_pressed(KeyCode::Space) {
                            game_state = GameState::Menu(MenuState::Landing);
                        }
                    }
                }

                player_1.draw(BLUE);
                player_2.draw(GREEN);
            }
            GameState::Game => {
                player_1.update(get_frame_time(), true);
                player_2.update(get_frame_time(), false);

                ball.update(get_frame_time(), level);

                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Pause;
                }

                clear_background(BLACK);
                player_1.draw(BLUE);
                player_2.draw(GREEN);

                // player_1_center.draw(YELLOW);
                // player_2_center.draw(YELLOW);
                ball.draw();

                //To detect collision between ball and the player(RECT)
                detect_collision(
                    &mut ball.dimention,
                    &mut ball.vel,
                    &player_1.rect,
                    &mut flying_emitter_world,
                    font,
                );
                detect_collision(
                    &mut ball.dimention,
                    &mut ball.vel,
                    &player_2.rect,
                    &mut flying_emitter_world,
                    font,
                );

                //To increase player's score
                let (returned_1_score, returned_2_score) =
                    detect_loss(&mut ball, &mut flying_emitter_world);

                if returned_1_score != 0 {
                    player_1_score += 1
                }
                if returned_2_score != 0 {
                    player_2_score += 1
                }

                //To Create Score board UI
                let p1_score = format!("{}", player_1_score);
                let p1_score_dimention =
                    measure_text(&p1_score, Some(font), PLAYER_SCORE_SIZE, 1.0);

                let p2_score = format!("{}", player_2_score);
                let p2_score_dimention =
                    measure_text(&p2_score, Some(font), PLAYER_SCORE_SIZE, 1.0);

                draw_header_text(&format!("score"), font);

                draw_text_ex(
                    &p1_score,
                    screen_width() * 0.3 - p1_score_dimention.width * 0.5,
                    80.0,
                    TextParams {
                        font,
                        font_size: PLAYER_SCORE_SIZE,
                        color: WHITE,
                        ..Default::default()
                    },
                );
                draw_description_text(&format!("First to {}", round), font, 90.0, 20);
                draw_description_text(&format!("Level {}", difficulty), font, 120.0, 20);
                draw_text_ex(
                    &p2_score,
                    screen_width() * 0.7 - p2_score_dimention.width * 0.5,
                    80.0,
                    TextParams {
                        font,
                        font_size: PLAYER_SCORE_SIZE,
                        color: WHITE,
                        ..Default::default()
                    },
                );
                draw_description_text(
                    &format!("Press ESC to pause"),
                    font,
                    screen_height() * 0.98,
                    20,
                );
                if player_1_score == round || player_2_score == round {
                    game_state = GameState::Complete
                }

                // flying_emitter_world.config.emitting = true;
            }
            GameState::Complete => {
                draw_header_text(&format!("Game Over"), font);
                if player_1_score > player_2_score {
                    draw_description_text(
                        &format!("Player 1 wins the game"),
                        font,
                        screen_height() * 0.3,
                        DESCRIPTION_FONT_SIZE,
                    );
                } else {
                    draw_description_text(
                        &format!("Player 2 wins the game"),
                        font,
                        screen_height() * 0.3,
                        DESCRIPTION_FONT_SIZE,
                    );
                }
                draw_description_text(
                    &format!("final score"),
                    font,
                    screen_height() * 0.4,
                    DESCRIPTION_FONT_SIZE,
                );
                draw_description_text(
                    &format!("{}   {}", player_1_score, player_2_score),
                    font,
                    screen_height() * 0.5,
                    PLAYER_SCORE_SIZE,
                );

                draw_description_text(
                    &format!("Press R to restart the game"),
                    font,
                    screen_height() * 0.6,
                    18,
                );
                draw_description_text(
                    &format!("Press Space to continue"),
                    font,
                    screen_height() * 0.7,
                    30,
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu(MenuState::Landing);
                    reset_game(
                        &mut player_1,
                        &mut player_2,
                        &mut player_1_score,
                        &mut player_2_score,
                        &mut round,
                        &mut level,
                        &mut difficulty,
                        &mut invalid_input,
                        &mut invalid_difficulty,
                    );
                }

                if is_key_pressed(KeyCode::R) {
                    game_state = GameState::Game;
                    restart_game(
                        &mut player_1,
                        &mut player_2,
                        &mut player_1_score,
                        &mut player_2_score,
                    )
                }
            }
            GameState::Pause => {
                draw_header_text(&format!("Game Paused"), font);
                draw_description_text(
                    &format!("Press Space to resume"),
                    font,
                    screen_height() * 0.5,
                    DESCRIPTION_FONT_SIZE,
                );
                draw_description_text(
                    &format!("Press R to restart the game"),
                    font,
                    screen_height() * 0.6,
                    20,
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
                if is_key_pressed(KeyCode::R) {
                    game_state = GameState::Game;
                    restart_game(
                        &mut player_1,
                        &mut player_2,
                        &mut player_1_score,
                        &mut player_2_score,
                    )
                }
            }
        }

        next_frame().await
    }
}
