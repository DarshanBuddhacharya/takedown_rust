use macroquad::prelude::{Rect, Vec2};

use macroquad::prelude::*;

//This function detects ball collision with respect to the player and changes the ball's velocity to opposite(In different direction)
pub fn detect_collision(a: &mut Circle, vel: &mut Vec2, b: &Rect) -> bool {
    let intersection = a.overlaps_rect(&*b);
    if intersection {
        vel.x *= -1.0;
    }
    return intersection;
}
