use macroquad::prelude::*;
use std::vec::Vec;
use core::num::diy_float::Fp;

struct FpsData {
    ringbuffer: Vec<u16>,
    ring_size: u8,
    index: u8,
}

impl FpsData {
    fn new(frame_window: u8) -> FpsData {
        let vec = Vec::with_capacity(frame_window.into());
        FpsData {
            ringbuffer: vec,
            ring_size: frame_window,
            index: 0
        }
    }

    fn add_fps(mut self, fps: u16) {
        if self.ringbuffer.len().into() < self.ring_size {
            self.ringbuffer.push(fps);
        } else if self.index >= (self.ring_size - 1) {
            self.index = 0;
            self.ringbuffer[0] = fps;
        } else {
            self.ringbuffer[self.index] = fps;
            self.index += 1;
        }
    }

    fn calculate_average(self) -> u16 {
        if self.ringbuffer.len() == 0 {
            0
        } else {
            let sum = self.ringbuffer.into_iter().fold(0u64, |b, x| {
                b + x.into()
            });
            (sum / self.ringbuffer.len()).into()
        }
    }

    fn add_and_average(mut self, fps : u16) -> u16 {
        self.add_fps(fps);
        self.calculate_average()
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut fps_data = FpsData::new(10);
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        let avg_fps = fps_data.add_and_average(get_fps().into());
        draw_text(avg_fps().as_str(), 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
