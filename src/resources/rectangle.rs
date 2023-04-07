use macroquad::prelude::*;

const RECT_SIZE: macroquad::math::Vec2 = Vec2::from_array([10.0, 150.0]);
const PLAYER_SPEED: f32 = 700.0;

pub struct Player {
    pub rect: Rect,
}

impl Player {
    pub fn new(postion_x: f32) -> Self {
        Self {
            rect: Rect::new(
                postion_x,
                screen_height() * 0.5 - RECT_SIZE.y * 0.5,
                RECT_SIZE.x,
                RECT_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, delta_time: f32, is_left: bool) {
        let mut y_move: f32 = 0.0;
        if is_left {
            if is_key_down(KeyCode::W) {
                y_move -= 1.0;
            }
            if is_key_down(KeyCode::S) {
                y_move += 1.0;
            }
        } else {
            if is_key_down(KeyCode::Up) {
                y_move -= 1.0;
            }
            if is_key_down(KeyCode::Down) {
                y_move += 1.0;
            }
        }
        self.rect.y += y_move * delta_time * PLAYER_SPEED;

        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
        }
        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
        draw_rectangle(
            self.rect.x,
            self.rect.center().y - 20.0 * 0.5,
            self.rect.w,
            20.0,
            YELLOW,
        );
    }
}
