use macroquad::input::KeyCode;
use macroquad::rand::gen_range;

pub struct FpsData {
    ringbuffer: Vec<u16>,
    ring_size: u8,
    index: u8,
}

impl FpsData {
    pub fn new(frame_window: u8) -> FpsData {
        let vec = Vec::with_capacity(frame_window.into());
        FpsData {
            ringbuffer: vec,
            ring_size: frame_window,
            index: 0
        }
    }

    fn add_fps(&mut self, fps: u16) {
        if self.ringbuffer.len() < (self.ring_size as usize) {
            self.ringbuffer.push(fps);
        } else if self.index >= (self.ring_size - 1) {
            self.index = 0;
            self.ringbuffer[0] = fps;
        } else {
            self.ringbuffer[self.index as usize] = fps;
            self.index += 1;
        }
    }

    fn calculate_average(&self) -> u16 {
        if self.ringbuffer.len() == 0 {
            0
        } else {
            let sum = self.ringbuffer.iter().fold(0u64, |b, &x| {
                b + u64::from(x)
            });
            (sum / (self.ringbuffer.len() as u64)).try_into().expect("Overflow calculating fps")
        }
    }

    pub fn add_and_average(&mut self, fps : u16) -> u16 {
        self.add_fps(fps);
        self.calculate_average()
    }
}

pub struct LetterStruct {
    pub text: String,
    pub keycode: KeyCode,
}

#[derive(Copy, Clone)]
pub enum Letter {
    H,
    J,
    K,
    L,
}

impl Letter {
    pub fn value(&self) -> LetterStruct {
        match *self {
            Letter::H => LetterStruct { text: "h".to_string(), keycode: KeyCode::H },
            Letter::J => LetterStruct { text: "j".to_string(), keycode: KeyCode::J },
            Letter::K => LetterStruct { text: "k".to_string(), keycode: KeyCode::K },
            Letter::L => LetterStruct { text: "l".to_string(), keycode: KeyCode::L },
        }
    }
}

pub struct GameState {
    pub letter_queue: Vec<Letter>,
    pub correct: Vec<(Letter, chrono::Duration)>,
    pub failed: Vec<(Letter, chrono::Duration)>,
}

pub trait Generator {
    fn generate(&mut self) -> Letter;
}

pub struct DumbGenerator {
    pub letters: Vec<Letter>

}

impl Generator for DumbGenerator {
    fn generate(&mut self) -> Letter {
        let high = self.letters.len() - 1;
        let choice = gen_range(0, high);
        self.letters[choice]
    }
}

impl GameState {
    pub fn generate_new_letters<T: Generator>(&mut self, n: u8, generator: &mut T) {
        let mut letter_queue = Vec::with_capacity(n as usize);
        for i in 0..n {
            letter_queue.push(generator.generate())
        }

        self.letter_queue = letter_queue;
        self.correct = Vec::new();
        self.failed = Vec::new();
    }

    pub fn is_complete(&self) -> bool {
        self.letter_queue.len() == 0
    }

    pub fn new<T: Generator>(generator: &mut T) -> GameState {
        let mut game_state = GameState{ letter_queue: vec![], correct: vec![], failed: vec![] };
        game_state.generate_new_letters(5, generator);
        game_state
    }
}

#[path = "../tests/test_fps.rs"]
#[cfg(test)]
mod test_fps;