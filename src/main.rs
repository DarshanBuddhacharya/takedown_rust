use crate::resources::rectangle::Player;
use macroquad::prelude::*;
use resources::ball::Ball;

mod resources;

#[macroquad::main("Takedown")]
async fn main() {
    let mut player_1 = Player::new(0.02);
    let mut player_2 = Player::new(0.92);

    let mut ball = Ball::new(vec2(screen_height() * 0.5, screen_width() * 0.5));

    loop {
        player_1.update(get_frame_time(), true);
        player_2.update(get_frame_time(), false);
        ball.update(get_frame_time());

        clear_background(BLACK);
        player_1.draw(BLUE);
        player_2.draw(GREEN);
        ball.draw();

        next_frame().await
    }
}
