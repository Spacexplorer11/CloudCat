use crate::get_responsive_size;
use macroquad::color::WHITE;
use macroquad::math::{Rect, vec2};
use macroquad::prelude::{DrawTextureParams, Texture2D, draw_texture_ex, screen_height};

pub(crate) struct Umbrella {
    // Umbrella variable reminds me of Squid Games...
    pub(crate) start_time: f64,
    pub(crate) texture: Texture2D,
}

impl Umbrella {
    pub(crate) async fn draw(&self) {
        let width = 32.0;
        let height = 32.0;

        draw_texture_ex(
            &self.texture,
            100.0,
            screen_height() - 20.0 - get_responsive_size(height) * 8.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    get_responsive_size(width) * 7.0,
                    get_responsive_size(height) * 8.0,
                )),
                source: Some(Rect {
                    x: 0.0,
                    y: 0.0,
                    w: width,
                    h: height,
                }),
                ..Default::default()
            },
        );
    }
}
