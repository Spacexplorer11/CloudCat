use macroquad::prelude::*;

#[macroquad::main("CloudCat")]
async fn main() {
    // Catty variables :3
    let mut cat_frame = 0;
    let mut cat_timer = 0.0;
    let cat_run_speed = 0.1;

    // Cloud variables ☁
    let mut cloud_frame = 0;
    let mut cloud_timer = 0.0;
    let mut cloud_x = 350.0;
    loop {
        clear_background(WHITE);

        draw_text("CloudCat", screen_width() * 0.45, 50.0, 50.0, DARKGRAY);

        (cloud_timer, cloud_frame) = draw_cloud(cloud_timer, cloud_frame, cloud_x).await;

        (cat_timer, cat_frame) = draw_cat(cat_timer, cat_frame, cat_run_speed).await;

        cloud_x -= 1.0;

        next_frame().await
    }
}

async fn draw_cat(mut timer: f32, mut frame: i32, cat_run_speed: f32) -> (f32, i32) {
    let cat: Texture2D = load_texture("assets/cat.png").await.unwrap();
    cat.set_filter(FilterMode::Nearest);

    let frame_width = 32.0;
    let frame_height = 32.0;
    draw_texture_ex(
        &cat,
        100.0,
        screen_height() - 200.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(frame_width * 5.0, frame_height * 5.0)),
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

async fn draw_cloud(mut timer: f32, mut frame: i32, cloud_x: f32) -> (f32, i32) {
    let cloud: Texture2D = load_texture("assets/cloud.png").await.unwrap();
    cloud.set_filter(FilterMode::Nearest);

    let fps = 0.1;

    let frame_width = 32.0;
    let frame_height = 32.0;
    draw_texture_ex(
        &cloud,
        cloud_x,
        screen_height() - 390.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(frame_width * 6.0, frame_height * 7.0)),
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
