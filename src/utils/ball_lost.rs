use macroquad::prelude::*;

use crate::resources::ball::Ball;

//This function detects if the ball's postion has reached the upmost left or right postion
//Then redirects the ball, changes its velocity and adds a number(NOT PLAYER SCORE)
pub fn detect_loss(a: &mut Ball) -> (u16, u16) {
    let mut p1_number = 0;
    let mut p2_number = 0;
    if a.dimention.x > screen_width() {
        p1_number += 1;
        a.dimention.x = screen_width() * 0.5;
        a.vel.x = -1.0;
    };
    if a.dimention.x < 0.0 {
        p2_number += 1;
        a.dimention.x = screen_width() * 0.5;
        a.vel.x = 1.0;
    };
    (p1_number, p2_number)
}
