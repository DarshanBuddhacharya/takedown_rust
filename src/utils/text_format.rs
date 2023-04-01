use macroquad::prelude::*;

const HEADER_FONT_SIZE: u16 = 35;

pub fn draw_header_text(text: &str, font: Font) {
    let dimentions = measure_text(text, Some(font), HEADER_FONT_SIZE, 1.0);
    draw_text_ex(
        text,
        screen_width() * 0.5 - dimentions.width * 0.5,
        55.0,
        TextParams {
            font,
            font_size: HEADER_FONT_SIZE,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub fn draw_description_text(text: &str, font: Font, y: f32, size: u16) {
    let dimentions = measure_text(text, Some(font), size, 1.0);
    draw_text_ex(
        text,
        screen_width() * 0.5 - dimentions.width * 0.5,
        y,
        TextParams {
            font,
            font_size: size,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub fn draw_error_text(font: Font) {
    let invalid_message = format!("Please Enter a valid number");
    let invalid_message_dimention = measure_text(&invalid_message, Some(font), 25, 1.0);
    draw_text_ex(
        &invalid_message,
        screen_width() * 0.5 - invalid_message_dimention.width * 0.5,
        screen_height() * 0.7,
        TextParams {
            font,
            font_size: 25,
            color: RED,
            ..Default::default()
        },
    );
}
