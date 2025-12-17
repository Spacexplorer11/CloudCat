use crate::get_responsive_size;
use macroquad::color::WHITE;
use macroquad::math::{Rect, vec2};
use macroquad::prelude::{
    DrawTextureParams, Texture2D, draw_texture_ex, get_frame_time, screen_height,
};

pub(crate) struct Cat {
    // Catty variables :3
    pub(crate) cat_frame: i32,
    pub(crate) cat_timer: f32,
    pub(crate) cat_run_speed: f32,
}

impl Cat {
    pub(crate) async fn draw_cat(&mut self, cat: &Texture2D) -> (f32, i32) {
        let frame_width = 32.0;
        let frame_height = 32.0;
        draw_texture_ex(
            &cat,
            100.0,
            screen_height() - 26.0 - get_responsive_size(frame_height) * 5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    get_responsive_size(frame_width) * 5.0,
                    get_responsive_size(frame_height) * 5.0,
                )),
                source: {
                    let inset = 0.5;
                    Some(Rect {
                        x: frame_width * self.cat_frame as f32 + inset,
                        y: 0.0 + inset,
                        w: frame_width - inset * 2.0,
                        h: frame_height - inset * 2.0,
                    })
                },
                ..Default::default()
            },
        );

        self.cat_timer += get_frame_time();
        if self.cat_timer > self.cat_run_speed {
            self.cat_timer = 0.0;
            self.cat_frame = (self.cat_frame + 1) % 3;
        }
        (self.cat_timer, self.cat_frame)
    }
}
