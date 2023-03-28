use macroquad::prelude::{Rect, Vec2};

use macroquad::prelude::*;

pub fn detect_collision(a: &mut Circle, vel: &mut Vec2, b: &Rect) -> bool {
    let intersection = a.overlaps_rect(&*b);
    if intersection {
        vel.x *= -1.0;
    }
    return intersection;
}
