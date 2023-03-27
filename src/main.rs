use crate::resources::rectangle::Player;
use macroquad::prelude::*;

mod resources;

#[macroquad::main("Takedown")]
async fn main() {
    let mut player_1 = Player::new(0.02);
    let mut player_2 = Player::new(0.92);
    loop {
        player_1.update(get_frame_time(), true);
        player_2.update(get_frame_time(), false);
        clear_background(BLACK);
        player_1.draw(BLUE);
        player_2.draw(GREEN);

        next_frame().await
    }
}
