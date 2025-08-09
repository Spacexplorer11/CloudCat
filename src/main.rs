use macroquad::prelude::*;

#[macroquad::main("CloudCat")]
async fn main() {
    let mut frame = 0;
    let mut timer = 0.0;
    loop {
        clear_background(WHITE);

        draw_text("CloudCat", screen_width() * 0.45, 50.0, 50.0, DARKGRAY);

        (timer, frame) = draw_cat(timer, frame).await;

        next_frame().await
    }
}

async fn draw_cat(mut timer: f32, mut frame: i32) -> (f32, i32) {
    let cat: Texture2D = load_texture("assets/cat.png").await.unwrap();
    cat.set_filter(FilterMode::Nearest);

    let cat_run_speed = 0.1;

    let frame_width = 32.0;
    let frame_height = 32.0;
    draw_texture_ex(
        &cat,
        100.0,
        100.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(frame_width * 10.0, frame_height * 10.0)),
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
