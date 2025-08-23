#[cfg(not(target_arch = "wasm32"))]
use ::rand::{Rng, rng};
use macroquad::prelude::*;
use quad_storage::STORAGE;

fn get_responsive_text_size(base_size: f32) -> f32 {
    let min_dimension = screen_width().min(screen_height());
    let scale_factor = (min_dimension / 800.0).max(0.4).min(1.5);
    base_size * scale_factor
}

struct HighscoreManager;

impl HighscoreManager {
    fn load() -> u32 {
        let storage = STORAGE.lock().unwrap();
        let zero: String = "0".parse().unwrap();
        storage.get("cloudcat_highscore").unwrap_or(zero).parse::<u32>().unwrap_or(0)
    }

    fn save(score: u32) {
        let mut storage = STORAGE.lock().unwrap();
        storage.set("cloudcat_highscore", &*score.to_string());
    }
}

#[macroquad::main("CloudCat")]
async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let mut rng = rng();

    let cat: Texture2D = load_texture("assets/cat.png").await.unwrap();
    cat.set_filter(FilterMode::Nearest);

    let cloud: Texture2D = load_texture("assets/cloud.png").await.unwrap();
    cloud.set_filter(FilterMode::Nearest);

    let floor_tex: Texture2D = load_texture("assets/floor.png").await.unwrap();
    floor_tex.set_filter(FilterMode::Nearest);

    let umbrella: Texture2D = load_texture("assets/umbrella.png").await.unwrap();
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
    let mut score = 0.0;
    let highscore = HighscoreManager::load();

    loop {
        if game_over {
            clear_background(RED);
            draw_text(
                "GAME OVER",
                screen_width() * 0.25,
                screen_height() * 0.5,
                get_responsive_text_size(100.0),
                DARKGRAY,
            );

            #[cfg(target_arch = "wasm32")]
            let restart_message = "Please refresh the page to play again";
            #[cfg(not(target_arch = "wasm32"))]
            let restart_message = "Please restart the game to play again";

            draw_text(
                restart_message,
                screen_width() * 0.2,
                screen_height() * 0.7,
                get_responsive_text_size(30.0),
                DARKGRAY,
            );
            next_frame().await;
            continue;
        }

        let dt = get_frame_time();

        clear_background(WHITE);

        let score_u32 = score as u32;

        draw_text(
            &format!("Score: {}", score_u32),
            screen_width() * 0.7,
            50.0,
            get_responsive_text_size(50.0),
            DARKGRAY,
        );

        if score_u32 < highscore {
            draw_text(
                &format!("Your highscore is {}", highscore),
                screen_width() * 0.01,
                50.0,
                get_responsive_text_size(50.0),
                DARKGRAY,
            );
        } else {
            draw_text(
                &format!("Your previous highscore was {}", highscore),
                0.0,
                50.0,
                get_responsive_text_size(40.0),
                DARKGRAY,
            );
        }

        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            if umbrella_start_time == 0.0 || get_time() - umbrella_start_time > 3.0 {
                umbrella_start_time = get_time();
            }
        }

        if cat_run_speed > 0.01 {
            cat_run_speed -= 0.0006 * dt;
        }

        let scroll_speed = 7.5 / cat_run_speed;

        cloud_x -= scroll_speed * dt;
        if cloud_x < -192.0 {
            #[cfg(not(target_arch = "wasm32"))]
            {
                cloud_x = screen_width() + rng.random_range(150.0..=200.0);
            }
            #[cfg(target_arch = "wasm32")]
            {
                cloud_x = screen_width() + rand::gen_range(150.0, 200.0);
            }
        }

        (cloud_timer, cloud_frame) = draw_cloud(&cloud, cloud_timer, cloud_frame, cloud_x).await;

        let umbrella_up = umbrella_start_time != 0.0 && (get_time() - umbrella_start_time) < 3.0;
        if umbrella_up {
            draw_umbrella(&umbrella).await;
        }

        (cat_timer, cat_frame) = draw_cat(&cat, cat_timer, cat_frame, cat_run_speed).await;

        draw_floor(&floor_tex, floor_x).await;

        floor_x -= scroll_speed * dt;
        if floor_x <= -screen_width() {
            floor_x = 0.0;
        }

        if (cloud_x <= 150.0 && cloud_x > 0.0) && !umbrella_up {
            game_over = true;
            if score_u32 > highscore {
                HighscoreManager::save(score_u32);
            }
        }

        score += 60.0 * dt;

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
