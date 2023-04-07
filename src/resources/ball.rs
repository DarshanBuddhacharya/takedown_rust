use macroquad::prelude::*;

const BALL_SIZE: f32 = 10.0;

pub struct Ball {
    pub dimention: Circle,
    pub vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            dimention: Circle::new(pos.x, pos.y, BALL_SIZE),
            vel: vec2(1.2, rand::gen_range(-1.0, 1.0)).normalize(),
        }
    }

    pub fn update(&mut self, delta_time: f32, level: f32) {
        self.dimention.x += self.vel.x * delta_time * level;
        self.dimention.y += self.vel.y * delta_time * level;

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
