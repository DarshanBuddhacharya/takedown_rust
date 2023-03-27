use macroquad::prelude::*;

const BALL_SIZE: f32 = 10.0;
const BALL_SPEED: f32 = 250.0;

pub struct Ball {
    dimention: Circle,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            dimention: Circle::new(pos.x, pos.y, BALL_SIZE),
            vel: vec2(1.0, rand::gen_range(-1.0, 1.0)).normalize(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.dimention.x += self.vel.x * delta_time * BALL_SPEED;
        self.dimention.y += self.vel.y * delta_time * BALL_SPEED;

        if self.dimention.y < 0.0 {
            self.vel.y = 1.0;
        }

        if self.dimention.y > screen_height() - self.dimention.r {
            self.vel.y = -1.0;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.dimention.x, self.dimention.y, self.dimention.r, WHITE);
    }
}
