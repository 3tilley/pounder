use macroquad::prelude::*;
use std::vec::Vec;
use pounder::{FpsData, DumbGenerator, GameState, Letter};

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut fps_data = FpsData::new(10);
    let mut gen = DumbGenerator{ letters: vec![Letter::H, Letter::J, Letter::K, Letter::L ]};
    let mut game_state = GameState::new(&mut gen);
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        let avg_fps = fps_data.add_and_average(get_fps().try_into().unwrap());
        draw_text(&avg_fps.to_string(), 20.0, 20.0, 30.0, DARKGRAY);

        if game_state.is_complete() {
            draw_text(format!("{} right, {} wrong!", game_state.correct.len(), game_state.failed.len()).as_str(), screen_width() / 2.0, screen_height() / 2.0, 30.0, DARKGRAY );
        } else {
            let next_letter = game_state.letter_queue.get(0);
            let letter_struct =
                match next_letter {
                    Some(letter) => letter.value(),
                    None => panic!("Shouldn't reach here"),
                };
            draw_text(letter_struct.text.as_str(), screen_width() / 2.0, screen_height() / 2.0, 60.0, DARKGRAY);
            let key = get_last_key_pressed();
            match key {
                Some(key) if letter_struct.keycode == key  => {
                    draw_text("nice", screen_width() / 2.0, screen_height() / 2.0, 60.0, DARKGRAY);
                    game_state.letter_queue.pop();
                }
                Some(key) => {
                    draw_text("nope", screen_width() / 2.0, screen_height() / 2.0, 60.0, DARKGRAY);
                    game_state.letter_queue.pop();
                }
                None => ()
            }
        }



        next_frame().await
    }
}
