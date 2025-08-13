use ::rand::{Rng, rng};
use macroquad::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;

#[macroquad::main("CloudCat")]
async fn main() {
    let exe_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();

    let mut rng = rng();

    let cat_path: PathBuf = exe_dir.join("assets/cat.png");
    let cat: Texture2D = load_texture(cat_path.to_str().unwrap()).await.unwrap();
    cat.set_filter(FilterMode::Nearest);

    let cloud_path: PathBuf = exe_dir.join("assets/cloud.png");
    let cloud: Texture2D = load_texture(cloud_path.to_str().unwrap()).await.unwrap();
    cloud.set_filter(FilterMode::Nearest);

    let floor_path: PathBuf = exe_dir.join("assets/floor.png");
    let floor_tex: Texture2D = load_texture(floor_path.to_str().unwrap()).await.unwrap();
    floor_tex.set_filter(FilterMode::Nearest);

    let umbrella_path: PathBuf = exe_dir.join("assets/umbrella.png");
    let umbrella: Texture2D = load_texture(umbrella_path.to_str().unwrap()).await.unwrap();
    umbrella.set_filter(FilterMode::Nearest);

    // Catty variables :3
    let mut cat_frame = 0;
    let mut cat_timer = 0.0;
    let mut cat_run_speed = 0.05;

    // Cloud variables ☁
    let mut cloud_frame = 0;
    let mut cloud_timer = 0.0;
    let mut cloud_x = 350.0;

    // Floor variable, just one :(
    let mut floor_x = 0.0;

    // Umbrella variable! Squid games....
    let mut umbrella_start_time = 0.0;

    // Game OVER RAWHHH >:)
    let mut game_over = false;

    // Score & Highscore RAWH
    let mut score = 0;
    let highscore = load_highscore();

    loop {
        if game_over {
            clear_background(RED);
            draw_text(
                "GAME OVER",
                screen_width() * 0.25,
                screen_height() * 0.5,
                100.0,
                DARKGRAY,
            );
            draw_text(
                "Please restart the game to play again",
                screen_width() * 0.2,
                screen_height() * 0.7,
                30.0,
                DARKGRAY,
            );
            next_frame().await;
            continue;
        }

        clear_background(WHITE);

        draw_text(
            &format!("Score: {}", score),
            screen_width() * 0.7,
            50.0,
            50.0,
            DARKGRAY,
        );

        if score < highscore {
            draw_text(
                &format!("Your highscore is {}", highscore),
                screen_width() * 0.01,
                50.0,
                50.0,
                DARKGRAY,
            );
        } else if score >= highscore {
            draw_text(
                &format!("Your previous highscore was {}", highscore),
                0.0,
                50.0,
                40.0,
                DARKGRAY,
            );
        }

        if is_key_pressed(KeyCode::Space) {
            if umbrella_start_time == 0.0 || get_time() - umbrella_start_time > 3.0 {
                umbrella_start_time = get_time();
            }
        }

        if cat_run_speed > 0.01 {
            cat_run_speed -= 0.00001;
        }

        cloud_x -= 0.125 / cat_run_speed;
        if cloud_x < -192.0 {
            cloud_x = screen_width() + rng.random_range(150.0..=200.0);
        }

        (cloud_timer, cloud_frame) = draw_cloud(&cloud, cloud_timer, cloud_frame, cloud_x).await;

        let umbrella_up = umbrella_start_time != 0.0 && (get_time() - umbrella_start_time) < 3.0;
        if umbrella_up {
            draw_umbrella(&umbrella).await;
        }

        (cat_timer, cat_frame) = draw_cat(&cat, cat_timer, cat_frame, cat_run_speed).await;

        draw_floor(&floor_tex, floor_x).await;

        floor_x -= 0.125 / cat_run_speed;
        if floor_x <= -screen_width() {
            floor_x = 0.0;
        }

        if (cloud_x <= 150.0 && cloud_x > 0.0) && !umbrella_up {
            game_over = true;
            fs::write("score.txt", score.to_string()).unwrap();
        }

        score += 1;

        next_frame().await;
    }
}

async fn draw_cat(
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

async fn draw_cloud(cloud: &Texture2D, mut timer: f32, mut frame: i32, cloud_x: f32) -> (f32, i32) {
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

async fn draw_floor(floor: &Texture2D, floor_x: f32) {
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

async fn draw_umbrella(umbrella: &Texture2D) {
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

fn load_highscore() -> i32 {
    match fs::read_to_string("score.txt") {
        Ok(s) => s.trim().parse::<i32>().unwrap_or(0),
        Err(_) => 0,
    }
}
