use ::rand::{Rng, thread_rng};
use macroquad::prelude::*;

#[macroquad::main("CloudCat")]
async fn main() {
    let mut rng = thread_rng();

    // Catty variables :3
    let mut cat_frame = 0;
    let mut cat_timer = 0.0;
    let cat_run_speed = 0.05;

    // Cloud variables ☁
    let mut cloud_frame = 0;
    let mut cloud_timer = 0.0;
    let mut cloud_x = 350.0;

    // Floor variable, just one :(
    let mut floor_x = 0.0;

    // Umbrella variable! Squid games....
    let mut umbrella_start_time = 0.0;

    // Score RAWH
    let mut score = 0;
    loop {
        clear_background(WHITE);

        draw_text(
            &format!("Score: {}", score),
            screen_width() * 0.7,
            50.0,
            50.0,
            DARKGRAY,
        );

        if is_key_pressed(KeyCode::Space) {
            if get_time() - umbrella_start_time > 3.0 {
                umbrella_start_time = get_time();
            } else if umbrella_start_time == 0.0 {
                umbrella_start_time = get_time();
            }
        }

        (cloud_timer, cloud_frame) = draw_cloud(cloud_timer, cloud_frame, cloud_x).await;

        if get_time() - umbrella_start_time < 3.0 && umbrella_start_time != 0.0 {
            draw_umbrella().await;
        }

        (cat_timer, cat_frame) = draw_cat(cat_timer, cat_frame, cat_run_speed).await;

        draw_floor(floor_x).await;

        cloud_x -= cat_run_speed * 50.0;
        if cloud_x < -192.0 {
            cloud_x = screen_width() + rng.gen_range(150.0..=200.0);
        }

        floor_x -= cat_run_speed * 50.0;
        if floor_x <= -screen_width() {
            floor_x = 0.0;
        }

        score += 1;
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

async fn draw_floor(floor_x: f32) {
    let floor: Texture2D = load_texture("assets/floor.png").await.unwrap();
    floor.set_filter(FilterMode::Nearest);

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

async fn draw_umbrella() {
    let umbrella: Texture2D = load_texture("assets/umbrella.png").await.unwrap();
    umbrella.set_filter(FilterMode::Nearest);

    let umbrella_width = 32.0;
    let umbrella_height = 32.0;

    draw_texture_ex(
        &umbrella,
        100.0,
        screen_height() - 270.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(umbrella_width * 7.0, umbrella_height * 8.0)),
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
