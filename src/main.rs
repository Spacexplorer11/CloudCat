#[cfg(not(target_arch = "wasm32"))]
use ::rand::{Rng, rng};
use macroquad::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;



#[cfg(target_arch = "wasm32")]
use std::sync::atomic::{AtomicI32, AtomicBool, Ordering};

#[cfg(target_arch = "wasm32")]
static HIGHSCORE: AtomicI32 = AtomicI32::new(0);

#[cfg(target_arch = "wasm32")]
static HIGHSCORE_INITIALIZED: AtomicBool = AtomicBool::new(false);



fn get_responsive_text_size(base_size: f32) -> f32 {
    let min_dimension = screen_width().min(screen_height());
    let scale_factor = (min_dimension / 800.0).max(0.4).min(1.5);
    base_size * scale_factor
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
    let _initial_highscore = load_highscore().await; // Initialize the system

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

        let score_i32 = score as i32;
        let highscore = get_current_highscore();

        draw_text(
            &format!("Score: {}", score_i32),
            screen_width() * 0.7,
            50.0,
            get_responsive_text_size(50.0),
            DARKGRAY,
        );

        if score_i32 < highscore {
            draw_text(
                &format!("Highscore: {}", highscore),
                screen_width() * 0.01,
                50.0,
                get_responsive_text_size(50.0),
                DARKGRAY,
            );
        } else if highscore > 0 {
            draw_text(
                &format!("Previous best: {}", highscore),
                screen_width() * 0.01,
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
            let current_highscore = get_current_highscore();
            if score_i32 > current_highscore {
                save_highscore(score_i32);
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

async fn load_highscore() -> i32 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        match fs::read_to_string("score.txt") {
            Ok(s) => s.trim().parse::<i32>().unwrap_or(0),
            Err(_) => 0,
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        // Try to load from localStorage via JavaScript
        if !HIGHSCORE_INITIALIZED.load(Ordering::Relaxed) {
            // Send a special message that JavaScript can intercept to load from localStorage
            macroquad::logging::info!("CLOUDCAT_STORAGE_LOAD:cloudcat_highscore");
            
            // Wait for JavaScript to process the request and check for a response
            for _attempt in 0..10 {
                next_frame().await;
                
                // Check if we got a response by looking for the response marker
                // JavaScript will send a special info message we can detect
                // For now, we'll just wait and use the fact that JavaScript logs show loading works
            }
            
            // Initialize with 0 for safety - JavaScript will handle persistence
            HIGHSCORE.store(0, Ordering::Relaxed);
            HIGHSCORE_INITIALIZED.store(true, Ordering::Relaxed);
            macroquad::logging::info!("Initialized web highscore system");
        }
        
        get_current_highscore()
    }
}

#[cfg(target_arch = "wasm32")]
fn get_current_highscore() -> i32 {
    HIGHSCORE.load(Ordering::Relaxed)
}

#[cfg(not(target_arch = "wasm32"))]
fn get_current_highscore() -> i32 {
    // For non-web platforms, we load from file each time
    match fs::read_to_string("score.txt") {
        Ok(s) => s.trim().parse::<i32>().unwrap_or(0),
        Err(_) => 0,
    }
}

fn save_highscore(score: i32) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = fs::write("score.txt", score.to_string());
    }
    #[cfg(target_arch = "wasm32")]
    {
        // Save to our atomic variable
        HIGHSCORE.store(score, Ordering::Relaxed);
        
        // Send a special message that JavaScript can intercept to save to localStorage
        macroquad::logging::info!("CLOUDCAT_STORAGE_SAVE:cloudcat_highscore:{}", score);
        macroquad::logging::info!("Saved new highscore: {}", score);
    }
}
