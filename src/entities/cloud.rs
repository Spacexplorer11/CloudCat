use crate::get_responsive_size;
use macroquad::color::WHITE;
use macroquad::math::{Rect, vec2};
use macroquad::prelude::{
    DrawTextureParams, Texture2D, draw_texture_ex, get_frame_time, screen_height,
};

pub(crate) struct Cloud {
    // Cloud variables ☁
    pub(crate) cloud_frame: i32,
    pub(crate) cloud_timer: f32,
    pub(crate) cloud_x: f32,
}

impl Cloud {
    pub(crate) async fn draw_cloud(
        cloud: &Texture2D,
        mut timer: f32,
        mut frame: i32,
        cloud_x: f32,
    ) -> (f32, i32) {
        let fps = 0.1;
        let frame_width = 32.0;
        let frame_height = 32.0;
        draw_texture_ex(
            &cloud,
            cloud_x,
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
                    x: frame_width * frame as f32,
                    y: 0.0,
                    w: frame_width,
                    h: frame_height,
                }),
                ..Default::default()
            },
        );

        timer += get_frame_time();
        if timer > fps {
            timer = 0.0;
            frame = (frame + 1) % 7;
        }
        (timer, frame)
    }
}
