use crate::entities::Animation;
use crate::get_responsive_size;
use macroquad::color::WHITE;
use macroquad::math::{Rect, vec2};
use macroquad::prelude::{
    DrawTextureParams, Texture2D, draw_texture_ex, get_frame_time, screen_height,
};

pub(crate) struct Cloud<'a> {
    // Cloud variables ☁
    pub(crate) frame: i32,
    pub(crate) timer: f32,
    pub(crate) x: f32,
    pub(crate) texture: &'a Texture2D,
}

impl Animation for Cloud<'_> {
    async fn draw(&mut self) -> (f32, i32) {
        let fps = 0.1;
        let frame_width = 32.0;
        let frame_height = 32.0;
        draw_texture_ex(
            self.texture,
            self.x,
            screen_height()
                - 30.0
                - get_responsive_size(frame_height) * 7.0
                - get_responsive_size(32.0) * 5.0, // to take away the catty's height too
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    get_responsive_size(frame_width) * 6.0,
                    get_responsive_size(frame_height) * 7.0,
                )),
                source: Some(Rect {
                    x: frame_width * self.frame as f32,
                    y: 0.0,
                    w: frame_width,
                    h: frame_height,
                }),
                ..Default::default()
            },
        );

        self.timer += get_frame_time();
        if self.timer > fps {
            self.timer = 0.0;
            self.frame = (self.frame + 1) % 7;
        }
        (self.timer, self.frame)
    }
}
