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

#[path = "../tests/test_fps.rs"]
#[cfg(test)]
mod test_fps;