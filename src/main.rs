use crate::resources::rectangle::Player;
use macroquad::prelude::*;

mod resources;

#[macroquad::main("Takedown")]
async fn main() {
    let player_1 = Player::new(0.02);
    let player_2 = Player::new(0.92);
    loop {
        clear_background(BLACK);
        player_1.draw(BLUE);
        player_2.draw(GREEN);

        next_frame().await
    }
}
