use crate::resources::rectangle::Player;
use macroquad::prelude::*;
use resources::ball::Ball;
use utils::{
    ball_lost::detect_loss,
    collision::detect_collision,
    text_format::{draw_description_text, draw_header_text},
};

mod resources;
mod utils;

const DESCRIPTION_FONT_SIZE: u16 = 32;
const PLAYER_SCORE_SIZE: u16 = 35;
const GAMEOVER_SCORE: i32 = 3;

enum GameState {
    Menu,
    Game,
    Pause,
    Complete,
}

fn reset_game(
    player_1: &mut Player,
    player_2: &mut Player,
    player_1_score: &mut i32,
    player_2_score: &mut i32,
) {
    *player_1 = Player::new(screen_width() * 0.96);
    *player_2 = Player::new(screen_width() * 0.02);
    *player_1_score = 0;
    *player_2_score = 0;
}

#[macroquad::main("Takedown")]
async fn main() {
    let font = load_ttf_font("./font/Anton-Regular.ttf").await.unwrap();

    let mut game_state = GameState::Menu;

    let mut player_1_score = 0;
    let mut player_2_score = 0;

    let mut ball = Ball::new(vec2(screen_height() * 0.5, screen_width() * 0.5));

    let mut player_1 = Player::new(10.0);
    let mut player_2 = Player::new(screen_width() - 20.0);

    loop {
        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
                draw_header_text(&format!("TakeDown"), font);

                draw_description_text(
                    &format!("First to {} points wins the game", GAMEOVER_SCORE),
                    font,
                    screen_height() * 0.45,
                    25,
                );

                draw_description_text(
                    &format!("Press Space to start the game"),
                    font,
                    screen_height() * 0.55,
                    DESCRIPTION_FONT_SIZE,
                );

                player_1.draw(BLUE);
                player_2.draw(GREEN);
            }
            GameState::Game => {
                player_1.update(get_frame_time(), true);
                player_2.update(get_frame_time(), false);
                ball.update(get_frame_time());

                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Pause;
                }

                clear_background(BLACK);
                player_1.draw(BLUE);
                player_2.draw(GREEN);
                ball.draw();

                //To detect collision between ball and the player(RECT)
                detect_collision(&mut ball.dimention, &mut ball.vel, &player_1.rect);
                detect_collision(&mut ball.dimention, &mut ball.vel, &player_2.rect);

                //To increase player's score
                let (returned_1_score, returned_2_score) = detect_loss(&mut ball);

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
                if player_1_score == GAMEOVER_SCORE || player_2_score == GAMEOVER_SCORE {
                    game_state = GameState::Complete
                }
            }
            GameState::Complete => {
                draw_header_text(&format!("Game Over"), font);
                if player_1_score > player_2_score {
                    draw_description_text(
                        &format!("Player 1 wins the game"),
                        font,
                        screen_height() * 0.4,
                        DESCRIPTION_FONT_SIZE,
                    );
                } else {
                    draw_description_text(
                        &format!("Player 2 wins the game"),
                        font,
                        screen_height() * 0.4,
                        DESCRIPTION_FONT_SIZE,
                    );
                }
                draw_description_text(
                    &format!("final score"),
                    font,
                    screen_height() * 0.5,
                    DESCRIPTION_FONT_SIZE,
                );
                draw_description_text(
                    &format!("{}   {}", player_1_score, player_2_score),
                    font,
                    screen_height() * 0.6,
                    PLAYER_SCORE_SIZE,
                );

                draw_description_text(
                    &format!("Press Space to continue"),
                    font,
                    screen_height() * 0.7,
                    20,
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(
                        &mut player_1,
                        &mut player_2,
                        &mut player_1_score,
                        &mut player_2_score,
                    );
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
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
        }

        next_frame().await
    }
}
