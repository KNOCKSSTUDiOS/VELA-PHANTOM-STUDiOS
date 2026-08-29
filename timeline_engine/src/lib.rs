#[derive(Default, Clone)]
pub struct TimelineState {
    pub current_frame: usize,
}

impl TimelineState {
    pub fn step(&mut self) {
        self.current_frame += 1;
    }

    pub fn get_frame(&self) -> usize {
        self.current_frame
    }
}
