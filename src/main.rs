use macroquad::prelude::*;
use std::vec::Vec;
use pounder::FpsData;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut fps_data = FpsData::new(10);
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        let avg_fps = fps_data.add_and_average(get_fps().try_into().unwrap());
        draw_text(&avg_fps.to_string(), 20.0, 20.0, 30.0, DARKGRAY);
        draw_text("j", screen_width() / 2.0, screen_height() / 2.0, 60.0, DARKGRAY);
        let key = get_last_key_pressed();
        match key {
            Some(KeyCode::J) => {
                draw_text("nice", screen_width() / 2.0, screen_height() / 2.0, 60.0, DARKGRAY);
            }
            Some(key) => {
                draw_text("nope", screen_width() / 2.0, screen_height() / 2.0, 60.0, DARKGRAY);
            }
            None => ()
        }

        next_frame().await
    }
}
