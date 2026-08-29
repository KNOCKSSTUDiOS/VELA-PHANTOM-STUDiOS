pub struct TimelineState {
    pub current_frame: usize,
    pub total_frames: usize,
    pub playing: bool,
}

impl TimelineState {
    pub fn new() -> Self {
        Self { current_frame: 0, total_frames: 1000, playing: false }
    }
    pub fn tick_frame(&mut self) -> usize {
        if self.playing {
            self.current_frame = (self.current_frame + 1) % self.total_frames;
        }
        self.current_frame
    }
    pub fn seek_start(&mut self) { self.current_frame = 0; }
    pub fn seek_end(&mut self) { self.current_frame = self.total_frames - 1; }
    pub fn seek_frame(&mut self, frame: usize) { self.current_frame = frame.min(self.total_frames - 1); }
    pub fn toggle_playback(&mut self) { self.playing = !self.playing; }
    pub fn is_playing(&self) -> bool { self.playing }
    pub fn current_frame(&self) -> usize { self.current_frame }
    pub fn total_frames(&self) -> usize { self.total_frames }
}
