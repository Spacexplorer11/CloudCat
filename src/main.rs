mod highscore;
mod settings;

mod entities {
    pub mod cat;
    pub mod cloud;
    pub mod floor;
    pub mod umbrella;
}

use crate::entities::cat::Cat;
use crate::entities::cloud::Cloud;
use crate::entities::floor::Floor;
use crate::entities::umbrella::Umbrella;

#[cfg(not(target_arch = "wasm32"))]
use ::rand::{Rng, rng};
use macroquad::prelude::*;
use std::env;
use std::path::PathBuf;

pub(crate) fn get_responsive_size(base_size: f32) -> f32 {
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

fn get_dir_path() -> String {
    if cfg!(not(debug_assertions)) {
        let exe_path = env::current_exe().unwrap_or_else(|e| {
            eprintln!("Failed to get current executable path: {e}");
            PathBuf::new()
        });

        let exe_path = match exe_path.to_str() {
            Some(path) => {
                println!("The executable path is {}", path);
                path
            }
            _ => {
                eprintln!("The executable path couldn't be converted to a string");
                ""
            }
        };

        match String::from(exe_path).strip_suffix("cloudcat") {
            Some(path) => String::from(path),
            _ => String::new(),
        }
    } else {
        String::new()
    }
}

#[macroquad::main("CloudCat")]
async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let mut rng = rng();

    let cat_texture: Texture2D = load_texture(&*(get_dir_path() + "assets/cat.png"))
        .await
        .unwrap();
    cat_texture.set_filter(FilterMode::Nearest);

    let mut cat = Cat {
        cat_frame: 0,
        cat_timer: 0.0,
        cat_run_speed: 0.05,
    };

    let cloud_texture: Texture2D = load_texture(&*(get_dir_path() + "assets/cloud.png"))
        .await
        .unwrap();
    cloud_texture.set_filter(FilterMode::Nearest);

    let mut clouds: Vec<Cloud> = vec![Cloud {
        cloud_x: screen_width(),
        cloud_frame: 0,
        cloud_timer: 0.0,
    }];

    // Which clouds must be incinerated :(
    let mut clouds_to_die: Vec<usize> = vec![];

    let floor_texture: Texture2D = load_texture(&*(get_dir_path() + "assets/floor.png"))
        .await
        .unwrap();
    floor_texture.set_filter(FilterMode::Nearest);

    let mut floor = Floor { floor_x: 0.0 };

    let umbrella_texture: Texture2D = load_texture(&*(get_dir_path() + "assets/umbrella.png"))
        .await
        .unwrap();
    umbrella_texture.set_filter(FilterMode::Nearest);

    let mut umbrella = Umbrella {
        umbrella_start_time: 0.0,
    };

    let settings: Texture2D = load_texture(&*(get_dir_path() + "assets/settings.png"))
        .await
        .unwrap();
    settings.set_filter(FilterMode::Linear);

    let settings_menu: Texture2D = load_texture(&*(get_dir_path() + "assets/settings-menu.png"))
        .await
        .unwrap();
    settings_menu.set_filter(FilterMode::Nearest);

    let reset_buttons: Texture2D = load_texture(&*(get_dir_path() + "assets/reset_buttons.png"))
        .await
        .unwrap();
    reset_buttons.set_filter(FilterMode::Nearest);

    let github_icon: Texture2D = load_texture(&*(get_dir_path() + "assets/github_icon.png"))
        .await
        .unwrap();
    github_icon.set_filter(FilterMode::Linear);

    // Game OVER RAWHHH >:)
    let mut game_over = false;

    // Has da game started?? mrow :3
    let mut game_started = false;

    // Some lil title screen variables ~ idk what else to say bro
    let mut title_screen_frame: u16 = 0;
    let mut title_screen_opacity: f32 = 1.0;

    // Special extra TITLE cat object (our catty ain't an object but... yeah that's what we call the collection of variables I think... or is it Struct or idk man)
    let mut title_cat = Cat {
        cat_frame: 0,
        cat_timer: 0.0,
        cat_run_speed: 0.05,
    };
    let mut title_cat_x = 0.0;

    // let's throw in a singular settings menu variable to simplify my code
    let mut settings_menu_not_active;

    // Score & Highscore RAWH
    let mut score = 0.0;
    let mut highscore = highscore::HighscoreManager::load();

    loop {
        let score_u32 = score as u32;

        if title_screen_frame < 500 {
            clear_background(WHITE);
            draw_texture_ex(
                &cat_texture,
                title_cat_x,
                screen_height() * 0.2,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        get_responsive_size(32.0) * 5.0,
                        get_responsive_size(32.0) * 5.0,
                    )),
                    source: Some(Rect {
                        x: 32.0 * title_cat.cat_frame as f32,
                        y: 0.0,
                        w: 32.0,
                        h: 32.0,
                    }),
                    ..Default::default()
                },
            );
            title_cat.cat_timer += get_frame_time();
            if title_cat.cat_timer > title_cat.cat_run_speed {
                title_cat.cat_timer = 0.0;
                title_cat.cat_frame = (title_cat.cat_frame + 1) % 3;
            }
            draw_centred_text(
                "CloudCat",
                50.0,
                0.0,
                Color {
                    r: 0.31,
                    g: 0.31,
                    b: 0.31,
                    a: title_screen_opacity,
                },
                true,
            );
            draw_centred_text(
                "Made with <3 by Akaalroop Singh (spacexplorer11 on GitHub)",
                34.0,
                screen_height() * 0.6,
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: title_screen_opacity,
                },
                false,
            );

            title_screen_frame += 1;
            title_screen_opacity = (title_screen_opacity - 0.0016).max(0.0);
            title_cat_x += screen_width() / 500.0;
            next_frame().await;
            continue;
        }

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
            (game_started, highscore) = settings::Settings::draw_settings_and_github_icon(
                &settings,
                &settings_menu,
                &reset_buttons,
                &github_icon,
                highscore,
            )
            .await;
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

            draw_centred_text(
                "Thanks for playing CloudCat by Akaalroop Singh",
                34.0,
                screen_height() * 0.9,
                DARKGRAY,
                false,
            );

            draw_centred_text(
                "(spacexplorer11 on GitHub)",
                34.0,
                screen_height() * 0.95,
                DARKGRAY,
                false,
            );

            (settings_menu_not_active, highscore) =
                settings::Settings::draw_settings_and_github_icon(
                    &settings,
                    &settings_menu,
                    &reset_buttons,
                    &github_icon,
                    highscore,
                )
                .await;
            if settings_menu_not_active {
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
                20.0,
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
                get_responsive_size(50.0),
                DARKGRAY,
            );
        } else {
            draw_text(
                &format!("Your previous highscore was {}", highscore),
                20.0,
                110.0,
                get_responsive_size(40.0),
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
        for (i, cloud) in &mut clouds.iter_mut().enumerate() {
            cloud.cloud_x -= scroll_speed * dt;

            if cloud.cloud_x < -192.0 {
                if i == 0 {
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
                } else {
                    clouds_to_die.push(i);
                }
            }
        }

        for cloud_index in &mut clouds_to_die {
            clouds.remove(*cloud_index);
        }
        clouds_to_die.clear();

        for cloud in &mut clouds {
            (cloud.cloud_timer, cloud.cloud_frame) = cloud.draw_cloud(&cloud_texture).await;
        }

        let umbrella_up = umbrella.umbrella_start_time != 0.0
            && (get_time() - umbrella.umbrella_start_time) < 3.0;
        if umbrella_up {
            umbrella.draw_umbrella(&umbrella_texture).await;
        }

        (cat.cat_timer, cat.cat_frame) = cat.draw_cat(&cat_texture).await;

        floor.draw_floor(&floor_texture).await;

        floor.floor_x -= scroll_speed * dt;
        if floor.floor_x <= -screen_width() {
            floor.floor_x = 0.0;
        }

        for cloud in &clouds {
            // Check if cloud overlaps with cat position
            let cat_x = 100.0;
            let cat_width = get_responsive_size(32.0) * 5.0;
            let cloud_width = get_responsive_size(32.0) * 6.0;

            let cloud_right = cloud.cloud_x + cloud_width;
            let cat_right = cat_x + cat_width;

            if cloud.cloud_x < cat_right && cloud_right > cat_x && !umbrella_up {
                if score_u32 > highscore {
                    highscore::HighscoreManager::save(score_u32);
                    highscore = score_u32;
                }
                game_over = true;
            }
        }

        score += 60.0 * dt;
        #[cfg(not(target_arch = "wasm32"))]
        let rand_int = rng.random_range(1..=(50 * ((cat.cat_run_speed * 1000.0) as i32)));

        #[cfg(target_arch = "wasm32")]
        let rand_int = rand::gen_range(1, 50 * ((cat.cat_run_speed * 1000.0) as i32));

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
                clouds.push(Cloud {
                    cloud_x: new_cloud_x,
                    cloud_frame: 0,
                    cloud_timer: 0.0,
                });
            }
        }

        next_frame().await;
    }
}
