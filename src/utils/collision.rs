use macroquad::prelude::{Rect, Vec2};

use macroquad::prelude::*;

//This function detects ball collision with respect to the player and changes the ball's velocity to opposite(In different direction)
pub fn detect_collision(circle: &mut Circle, vel: &mut Vec2, rect: &Rect) -> bool {
    let intersection = circle.overlaps_rect(&*rect);

    let center = Rect::new(rect.x, rect.center().y - 20.0 * 0.5, 40.0, 30.0);

    let center_intersection = circle.overlaps_rect(&*&center);

    if intersection {
        // if center_intersection {
        //     debug!("oh yess daddy");
        //     vel.x *= -1.0;
        // }
        // debug!("-----------");

        if circle.point().x > rect.point().x {
            if center_intersection {
                debug!("-----------");
                vel.x *= -1.5;
            } else {
                vel.x = 1.0;
            }
        } else if circle.point().x < rect.point().x {
            if center_intersection {
                debug!("-----------");
                vel.x *= -1.5;
            } else {
                vel.x = -1.0;
            }
        }
    }
    return intersection;
}
