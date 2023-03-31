use crate::resources::rectangle::Player;
use macroquad::prelude::*;
use resources::ball::Ball;
use utils::collision::detect_collision;

mod resources;
mod utils;

const HEADER_FONT_SIZE: u16 = 40;
const PLAYER_SCORE_SIZE: u16 = 30;

#[macroquad::main("Takedown")]
async fn main() {
    let font = load_ttf_font("./font/Anton-Regular.ttf").await.unwrap();
    let mut player_1 = Player::new(0.02);
    let mut player_2 = Player::new(0.97);

    let player_1_score = 0;
    let player_2_score = 0;

    let mut ball = Ball::new(vec2(screen_height() * 0.5, screen_width() * 0.5));

    loop {
        player_1.update(get_frame_time(), true);
        player_2.update(get_frame_time(), false);
        ball.update(get_frame_time());

        detect_collision(&mut ball.dimention, &mut ball.vel, &player_1.rect);
        detect_collision(&mut ball.dimention, &mut ball.vel, &player_2.rect);

        clear_background(BLACK);
        player_1.draw(BLUE);
        player_2.draw(GREEN);
        ball.draw();

        let score_header = format!("score");
        let score_header_dimention = measure_text(&score_header, Some(font), HEADER_FONT_SIZE, 1.0);

        let p1_score = format!("{}", player_1_score);
        let p1_score_dimention = measure_text(&p1_score, Some(font), PLAYER_SCORE_SIZE, 1.0);

        let p2_score = format!("{}", player_2_score);
        let p2_score_dimention = measure_text(&p2_score, Some(font), PLAYER_SCORE_SIZE, 1.0);

        draw_text_ex(
            &score_header,
            screen_width() * 0.5 - score_header_dimention.width * 0.5,
            40.0,
            TextParams {
                font,
                font_size: HEADER_FONT_SIZE,
                color: WHITE,
                ..Default::default()
            },
        );

        draw_text_ex(
            &p1_score,
            screen_width() * 0.3 - p1_score_dimention.width * 0.5,
            80.0,
            TextParams {
                font,
                font_size: HEADER_FONT_SIZE,
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
                font_size: HEADER_FONT_SIZE,
                color: WHITE,
                ..Default::default()
            },
        );

        next_frame().await
    }
}
