pub struct TimelineState {
    frame: u64,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self { frame: 0 }
    }
}

impl TimelineState {
    pub fn step(&mut self) {
        self.frame += 1;
    }

    pub fn get_frame(&self) -> u64 {
        self.frame
    }
}
