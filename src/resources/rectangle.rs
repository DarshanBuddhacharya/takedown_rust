use macroquad::prelude::*;

const RECT_SIZE: macroquad::math::Vec2 = Vec2::from_array([40.0, 150.0]);

pub struct Player {
    rect: Rect,
}

impl Player {
    pub fn new(postion_x: f32) -> Self {
        Self {
            rect: Rect::new(
                screen_width() * postion_x,
                screen_height() * 0.5 - RECT_SIZE.y * 0.5,
                RECT_SIZE.x,
                RECT_SIZE.y,
            ),
        }
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}
