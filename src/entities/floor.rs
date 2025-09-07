use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::prelude::{
    DrawTextureParams, Texture2D, draw_texture_ex, screen_height, screen_width,
};

pub(crate) struct Floor {
    // Floor variables, yay now! :)
    pub(crate) texture: Texture2D,
    pub(crate) floor_x: f32,
}

impl Floor {
    pub(crate) async fn draw_floor(floor: &Texture2D, floor_x: f32) {
        let floor_width = screen_width();
        let floor_height = 24.0;

        for offset in [0.0, floor_width].iter() {
            draw_texture_ex(
                &floor,
                floor_x + *offset,
                screen_height() - 45.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(floor_width, floor_height)),
                    source: None,
                    ..Default::default()
                },
            );
        }
    }
}
