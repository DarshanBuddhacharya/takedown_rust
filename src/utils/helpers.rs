use macroquad::{prelude::KeyCode, window::screen_width};

use crate::resources::rectangle::Player;

// This function takes a KeyCode and returns an Option<i32>
// containing the corresponding digit if the key represents a
// valid digit, or None otherwise.
pub fn parse_digit(key: KeyCode) -> Option<i32> {
    match key {
        KeyCode::Key0 => Some(0),
        KeyCode::Key1 => Some(1),
        KeyCode::Key2 => Some(2),
        KeyCode::Key3 => Some(3),
        KeyCode::Key4 => Some(4),
        KeyCode::Key5 => Some(5),
        KeyCode::Key6 => Some(6),
        KeyCode::Key7 => Some(7),
        KeyCode::Key8 => Some(8),
        KeyCode::Key9 => Some(9),
        _ => None,
    }
}

//This Function resets the game to initial state
pub fn reset_game(
    player_1: &mut Player,
    player_2: &mut Player,
    player_1_score: &mut i32,
    player_2_score: &mut i32,
    invalid_input: &mut bool,
    level: &mut i32,
) {
    *player_1 = Player::new(10.0);
    *player_2 = Player::new(screen_width() - 20.0);
    *player_1_score = 0;
    *player_2_score = 0;
    *invalid_input = false;
    *level = 0;
}
