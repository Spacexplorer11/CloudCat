use crate::get_responsive_size;
use macroquad::color::WHITE;
use macroquad::math::{Rect, vec2};
use macroquad::prelude::{DrawTextureParams, Texture2D, draw_texture_ex, screen_height};

pub(crate) struct Umbrella {
    // Umbrella variable reminds me of Squid Games...
    pub(crate) umbrella_start_time: f64,
}

impl Umbrella {
    pub(crate) async fn draw_umbrella(umbrella: &Texture2D) {
        let umbrella_width = 32.0;
        let umbrella_height = 32.0;

        draw_texture_ex(
            &umbrella,
            100.0,
            screen_height() - 20.0 - get_responsive_size(umbrella_height) * 8.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    get_responsive_size(umbrella_width) * 7.0,
                    get_responsive_size(umbrella_height) * 8.0,
                )),
                source: Some(Rect {
                    x: 0.0,
                    y: 0.0,
                    w: umbrella_width,
                    h: umbrella_height,
                }),
                ..Default::default()
            },
        );
    }
}
