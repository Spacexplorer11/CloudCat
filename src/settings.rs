use crate::get_responsive_size;
use macroquad::color::WHITE;
use macroquad::input::{MouseButton, is_mouse_button_pressed, mouse_position};
use macroquad::math::{Rect, vec2};
use macroquad::prelude::{
    DrawTextureParams, Texture2D, clear_background, draw_texture_ex, next_frame, screen_height,
    screen_width,
};

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

    pub(crate) async fn settings_menu(settings_menu: &Texture2D) {
        loop {
            clear_background(WHITE);
            draw_texture_ex(
                settings_menu,
                screen_width() * 0.5 - (get_responsive_size(32.0) * 15.0) * 0.5,
                screen_height() * 0.5 - (get_responsive_size(32.0) * 15.0) * 0.5,
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
            if is_mouse_button_pressed(MouseButton::Left) {
                if Self::is_settings_exit_clicked() {
                    break;
                }
            }
            next_frame().await;
        }
    }
}
