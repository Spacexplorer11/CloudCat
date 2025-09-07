use crate::get_responsive_size;
use macroquad::color::WHITE;
use macroquad::math::{Rect, vec2};
use macroquad::prelude::{
    DrawTextureParams, Texture2D, draw_texture_ex, get_frame_time, screen_height,
};

pub(crate) struct Cat {
    // Catty variables :3
    pub(crate) texture: Texture2D,
    pub(crate) cat_frame: i32,
    pub(crate) cat_timer: f32,
    pub(crate) cat_run_speed: f32,
}

impl Cat {
    pub(crate) async fn draw_cat(
        cat: &Texture2D,
        mut timer: f32,
        mut frame: i32,
        cat_run_speed: f32,
    ) -> (f32, i32) {
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
                source: Some(Rect {
                    x: frame_width * frame as f32,
                    y: 0.0,
                    w: frame_width,
                    h: frame_height,
                }),
                ..Default::default()
            },
        );

        timer += get_frame_time();
        if timer > cat_run_speed {
            timer = 0.0;
            frame = (frame + 1) % 3;
        }
        (timer, frame)
    }
}
