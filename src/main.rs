mod highscore;
mod settings;

mod entities {
    pub mod cat;
    pub mod cloud;
    pub mod floor;
    pub mod umbrella;
}

use crate::entities::cat;
use crate::entities::cloud;
use crate::entities::floor;
use crate::entities::umbrella;

#[cfg(not(target_arch = "wasm32"))]
use ::rand::{Rng, rng};
use macroquad::prelude::*;

fn get_responsive_size(base_size: f32) -> f32 {
    let min_dimension = screen_width().min(screen_height());
    let scale_factor = min_dimension * 0.0013;
    base_size * scale_factor
}

fn draw_centred_text(text: &str, base_font_size: f32, y: f32, colour: Color, centre_y: bool) {
    let font_size = get_responsive_size(base_font_size);
    let details = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - details.width) / 2.0;

    if centre_y {
        let y = (screen_height() - details.height) / 2.0;
        draw_text(text, x, y, font_size, colour);
        return;
    }
    draw_text(text, x, y, font_size, colour);
}

#[macroquad::main("CloudCat")]
async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let mut rng = rng();

    let cat_texture = load_texture("assets/cat.png").await.unwrap();
    cat_texture.set_filter(FilterMode::Nearest);

    let mut cat = cat::Cat {
        texture: cat_texture,
        cat_frame: 0,
        cat_timer: 0.0,
        cat_run_speed: 0.05,
    };

    let cloud_texture: Texture2D = load_texture("assets/cloud.png").await.unwrap();
    cloud_texture.set_filter(FilterMode::Nearest);

    let mut clouds: Vec<cloud::Cloud> = vec![cloud::Cloud {
        cloud_x: screen_width(),
        cloud_frame: 0,
        cloud_timer: 0.0,
    }];

    let floor_texture: Texture2D = load_texture("assets/floor.png").await.unwrap();
    floor_texture.set_filter(FilterMode::Nearest);

    let mut floor = floor::Floor {
        texture: floor_texture,
        floor_x: 0.0,
    };

    let umbrella_texture: Texture2D = load_texture("assets/umbrella.png").await.unwrap();
    umbrella_texture.set_filter(FilterMode::Nearest);

    let mut umbrella = umbrella::Umbrella {
        umbrella_start_time: 0.0,
    };

    let settings: Texture2D = load_texture("assets/settings.png").await.unwrap();
    settings.set_filter(FilterMode::Linear);

    let settings_menu: Texture2D = load_texture("assets/settings-menu.png").await.unwrap();
    settings_menu.set_filter(FilterMode::Nearest);

    // Game OVER RAWHHH >:)
    let mut game_over = false;

    // Has da game started?? mrow :3
    let mut game_started = false;

    // Score & Highscore RAWH
    let mut score = 0.0;
    let mut highscore = highscore::HighscoreManager::load();

    loop {
        let score_u32 = score as u32;

        if !game_started {
            clear_background(WHITE);
            draw_centred_text(
                "Please click/touch/hit space to put up the umbrella to protect your cat.",
                27.0,
                screen_height() * 0.3,
                DARKGRAY,
                false,
            );
            draw_centred_text(
                "The umbrella lasts 3 SECONDS",
                38.0,
                screen_height() * 0.4,
                RED,
                false,
            );
            draw_centred_text(
                "The aim of the game is not let your cat get touched by rain",
                34.0,
                screen_height() * 0.5,
                DARKGRAY,
                false,
            );
            draw_centred_text(
                "Click any key, tap or click anywhere to start the game",
                34.0,
                screen_height() * 0.6,
                DARKGRAY,
                false,
            );

            draw_texture_ex(
                &settings,
                screen_width() - get_responsive_size(32.0) * 2.5,
                screen_height() - get_responsive_size(32.0) * 2.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        get_responsive_size(32.0) * 2.5,
                        get_responsive_size(32.0) * 2.5,
                    )),
                    source: Some(Rect {
                        x: 0.0,
                        y: 0.0,
                        w: 32.0,
                        h: 32.0,
                    }),
                    ..Default::default()
                },
            );
            if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                if settings::Settings::is_settings_clicked() {
                    settings::Settings::settings_menu(&settings_menu).await;
                } else {
                    game_started = true;
                }
            }
            next_frame().await;
            continue;
        }
        if game_over {
            clear_background(RED);
            draw_centred_text("GAME OVER", 100.0, 0.0, DARKGRAY, true);
            draw_centred_text(
                &format!("Your score was {}", score_u32),
                50.0,
                screen_height() * 0.6,
                DARKGRAY,
                false,
            );

            #[cfg(target_arch = "wasm32")]
            let restart_message = "Please tap/click/hit space or refresh to play again";
            #[cfg(not(target_arch = "wasm32"))]
            let restart_message = "Please tap/click/hit space or restart the game to play again";

            draw_centred_text(
                restart_message,
                30.0,
                screen_height() * 0.7,
                DARKGRAY,
                false,
            );

            draw_texture_ex(
                &settings,
                screen_width() - get_responsive_size(32.0) * 2.5,
                screen_height() - get_responsive_size(32.0) * 2.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        get_responsive_size(32.0) * 2.5,
                        get_responsive_size(32.0) * 2.5,
                    )),
                    source: Some(Rect {
                        x: 0.0,
                        y: 0.0,
                        w: 32.0,
                        h: 32.0,
                    }),
                    ..Default::default()
                },
            );

            if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                if settings::Settings::is_settings_clicked() {
                    settings::Settings::settings_menu(&settings_menu).await;
                } else {
                    // Catty
                    cat.cat_frame = 0;
                    cat.cat_timer = 0.0;
                    cat.cat_run_speed = 0.05;

                    // Cloudy
                    for cloud in &mut clouds {
                        cloud.cloud_x = screen_width();
                        cloud.cloud_frame = 0;
                        cloud.cloud_timer = 0.0;
                    }

                    // Floorrrrrrr
                    floor.floor_x = 0.0;

                    // Umbrellaaaaaaaa
                    umbrella.umbrella_start_time = 0.0;

                    // Let's go back to the start!
                    game_over = false;
                    game_started = false;
                    highscore = highscore::HighscoreManager::load();
                    score = 0.0;
                    continue;
                }
            }
            next_frame().await;
            continue;
        }

        let dt = get_frame_time();

        clear_background(WHITE);

        #[cfg(not(target_arch = "wasm32"))]
        draw_text(
            &format!("Score: {}", score_u32),
            screen_width() * 0.7,
            50.0,
            get_responsive_size(50.0),
            DARKGRAY,
        );

        #[cfg(target_arch = "wasm32")]
        draw_text(
            &format!("Score: {}", score_u32),
            screen_width() * 0.7,
            110.0,
            get_responsive_size(50.0),
            DARKGRAY,
        );

        #[cfg(not(target_arch = "wasm32"))]
        if score_u32 < highscore {
            draw_text(
                &format!("Your highscore is {}", highscore),
                screen_width() * 0.01,
                50.0,
                get_responsive_size(50.0),
                DARKGRAY,
            );
        } else {
            draw_text(
                &format!("Your previous highscore was {}", highscore),
                0.0,
                50.0,
                get_responsive_size(40.0),
                DARKGRAY,
            );
        }

        #[cfg(target_arch = "wasm32")]
        if score_u32 < highscore {
            draw_text(
                &format!("Your highscore is {}", highscore),
                screen_width() * 0.01,
                110.0,
                crate::get_responsive_size(50.0),
                DARKGRAY,
            );
        } else {
            draw_text(
                &format!("Your previous highscore was {}", highscore),
                0.0,
                110.0,
                crate::get_responsive_size(40.0),
                DARKGRAY,
            );
        }

        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            if umbrella.umbrella_start_time == 0.0
                || get_time() - umbrella.umbrella_start_time > 3.0
            {
                umbrella.umbrella_start_time = get_time();
            }
        }

        if cat.cat_run_speed > 0.01 {
            cat.cat_run_speed -= 0.0006 * dt;
        }

        let scroll_speed = 7.5 / cat.cat_run_speed;

        let mut positions: Vec<f32> = clouds.iter().map(|cloud| cloud.cloud_x).collect();
        for cloud in &mut clouds {
            cloud.cloud_x -= scroll_speed * dt;

            if cloud.cloud_x < -192.0 {
                #[cfg(not(target_arch = "wasm32"))]
                let mut new_x = screen_width() + rng.random_range(150.0..=200.0);
                #[cfg(target_arch = "wasm32")]
                let mut new_x = screen_width() + rand::gen_range(150.0, 200.0);

                let min_spacing = get_responsive_size(32.0) * 12.0;

                for &pos in &positions {
                    if (pos - new_x).abs() < min_spacing {
                        new_x = pos + min_spacing;
                    }
                }

                cloud.cloud_x = new_x;
                cloud.cloud_frame = 0;
                cloud.cloud_timer = 0.0;

                positions.push(new_x);
            }
        }

        for cloud in &mut clouds {
            (cloud.cloud_timer, cloud.cloud_frame) = cloud::Cloud::draw_cloud(
                &cloud_texture,
                cloud.cloud_timer,
                cloud.cloud_frame,
                cloud.cloud_x,
            )
            .await;
        }

        let umbrella_up = umbrella.umbrella_start_time != 0.0
            && (get_time() - umbrella.umbrella_start_time) < 3.0;
        if umbrella_up {
            umbrella::Umbrella::draw_umbrella(&umbrella_texture).await;
        }

        (cat.cat_timer, cat.cat_frame) = cat::Cat::draw_cat(
            &cat.texture,
            cat.cat_timer,
            cat.cat_frame,
            cat.cat_run_speed,
        )
        .await;

        floor::Floor::draw_floor(&floor.texture, floor.floor_x).await;

        floor.floor_x -= scroll_speed * dt;
        if floor.floor_x <= -screen_width() {
            floor.floor_x = 0.0;
        }

        for cloud in &clouds {
            if (cloud.cloud_x <= 150.0 && cloud.cloud_x > 0.0) && !umbrella_up {
                game_over = true;
                if score_u32 > highscore {
                    highscore::HighscoreManager::save(score_u32);
                }
            }
        }

        score += 60.0 * dt;
        #[cfg(not(target_arch = "wasm32"))]
        let rand_int = rng.random_range(1..=1000);

        #[cfg(target_arch = "wasm32")]
        let rand_int = rand::gen_range(1, 1000);

        if rand_int == 11 {
            #[cfg(not(target_arch = "wasm32"))]
            let new_cloud_x = screen_width() + rng.random_range(150.0..=200.0);

            #[cfg(target_arch = "wasm32")]
            let new_cloud_x = screen_width() + rand::gen_range(150.0, 200.0);

            let mut too_close_cloud = false;
            for cloud in &clouds {
                if (cloud.cloud_x - new_cloud_x).abs() <= get_responsive_size(32.0) * 20.0 {
                    too_close_cloud = true;
                    break;
                }
            }
            if !too_close_cloud {
                clouds.push(cloud::Cloud {
                    cloud_x: new_cloud_x,
                    cloud_frame: 0,
                    cloud_timer: 0.0,
                });
            }
        }

        next_frame().await;
    }
}
