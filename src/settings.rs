use crate::{draw_centred_text, get_responsive_size, highscore};
use macroquad::color::{BLACK, WHITE};
use macroquad::input::{
    KeyCode, MouseButton, is_key_pressed, is_mouse_button_pressed, mouse_position,
};
use macroquad::math::{Rect, vec2};
use macroquad::prelude::{
    DrawTextureParams, Texture2D, clear_background, draw_texture_ex, next_frame, screen_height,
    screen_width,
};
use macroquad::text::draw_text;

pub(crate) struct Settings;

impl Settings {
    pub(crate) fn is_settings_clicked() -> bool {
        let (mx, my) = mouse_position();
        let (settings_x, settings_y) = (
            screen_width() - get_responsive_size(32.0) * 2.5,
            screen_height() - get_responsive_size(32.0) * 2.5,
        );
        if mx >= settings_x
            && mx <= settings_x + get_responsive_size(32.0) * 2.5
            && my >= settings_y
            && my <= settings_y + get_responsive_size(32.0) * 2.5
        {
            return true;
        }
        false
    }

    pub(crate) async fn draw_settings_icon(
        settings_icon: &Texture2D,
        settings_menu: &Texture2D,
        reset_button: &Texture2D,
        mut highscore: u32,
    ) -> (bool, u32) {
        draw_texture_ex(
            &settings_icon,
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
            if Self::is_settings_clicked() {
                highscore = Self::settings_menu(&settings_menu, &reset_button, highscore).await;
                (false, highscore)
            } else {
                (true, highscore)
            }
        } else {
            (false, highscore)
        }
    }

    pub(crate) fn is_settings_exit_clicked() -> bool {
        let menu_size = get_responsive_size(32.0) * 15.0;
        let menu_x = screen_width() * 0.5 - menu_size * 0.5;
        let menu_y = screen_height() * 0.5 - menu_size * 0.5;

        let button_size = menu_size / 5.0;

        let close_x = menu_x + menu_size - button_size;
        let close_y = menu_y;

        let (mx, my) = mouse_position();

        if mx >= close_x
            && mx <= close_x + button_size
            && my >= close_y
            && my <= close_y + button_size
        {
            return true;
        }
        false
    }

    pub(crate) fn is_reset_highscore_clicked() -> bool {
        let button_size = get_responsive_size(32.0) * 5.0;
        let button_x = screen_width() * 0.56
            - (get_responsive_size(32.0) * 15.0) * 0.5
            - get_responsive_size(20.0);
        let button_y = screen_height() * 0.67
            - (get_responsive_size(32.0) * 15.0) * 0.5
            - get_responsive_size(20.0);

        let (mx, my) = mouse_position();

        if mx >= button_x
            && mx <= button_x + button_size
            && my >= button_y
            && my <= button_y + button_size
        {
            return true;
        }
        false
    }

    pub(crate) async fn settings_menu(
        settings_menu: &Texture2D,
        reset_button: &Texture2D,
        mut highscore: u32,
    ) -> u32 {
        let mut reset_state: f32 = 0.0;
        loop {
            let menu_x: f32 = screen_width() * 0.5 - (get_responsive_size(32.0) * 15.0) * 0.5;
            let menu_y: f32 = screen_height() * 0.5 - (get_responsive_size(32.0) * 15.0) * 0.5;
            clear_background(WHITE);
            draw_texture_ex(
                settings_menu,
                menu_x,
                menu_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        get_responsive_size(32.0) * 15.0,
                        get_responsive_size(32.0) * 15.0,
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

            draw_texture_ex(
                reset_button,
                menu_x + get_responsive_size(30.0),
                menu_y + get_responsive_size(100.0),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        get_responsive_size(32.0) * 5.0,
                        get_responsive_size(32.0) * 5.0,
                    )),
                    source: Some(Rect {
                        x: 32.0 * reset_state,
                        y: 0.0,
                        w: 32.0,
                        h: 32.0,
                    }),
                    ..Default::default()
                },
            );

            draw_text(
                "Reset your highscore",
                menu_x + get_responsive_size(30.0) + get_responsive_size(170.0),
                menu_y + get_responsive_size(185.0),
                get_responsive_size(25.0),
                BLACK,
            );

            draw_centred_text(
                "Check out the github repo:",
                35.0,
                screen_height() * 0.45
                    - (get_responsive_size(32.0) * 15.0) * 0.5
                    - get_responsive_size(20.0),
                BLACK,
                false,
            );

            draw_centred_text(
                "https://github.com/spacexplorer11/cloudcat",
                23.0,
                screen_height() * 0.5
                    - (get_responsive_size(32.0) * 15.0) * 0.5
                    - get_responsive_size(20.0),
                BLACK,
                false,
            );
            if is_mouse_button_pressed(MouseButton::Left) {
                if Self::is_settings_exit_clicked() {
                    return highscore;
                } else if Self::is_reset_highscore_clicked() {
                    match reset_state {
                        0.0 => reset_state = 1.0,
                        1.0 => {
                            highscore::HighscoreManager::save(0);
                            highscore = 0;
                            reset_state = 2.0;
                        }
                        _ => {}
                    }
                }
            }

            if is_key_pressed(KeyCode::Escape) {
                return highscore;
            }
            next_frame().await;
        }
    }
}
