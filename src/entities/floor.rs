use macroquad::color::WHITE;
use macroquad::math::vec2;
use macroquad::prelude::{
    DrawTextureParams, Texture2D, draw_texture_ex, screen_height, screen_width,
};

pub(crate) struct Floor {
    // Floor variables now! No longer :(  now :)
    pub(crate) x: f32,
    pub(crate) texture: Texture2D,
}

impl Floor {
    pub(crate) async fn draw_floor(&mut self) {
        let width = screen_width();
        let height = 24.0;

        for offset in [0.0, width].iter() {
            draw_texture_ex(
                &self.texture,
                self.x + *offset,
                screen_height() - 45.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(width, height)),
                    source: None,
                    ..Default::default()
                },
            );
        }
    }
}
