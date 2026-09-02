pub struct RecognitionEngine {
    pub is_listening: bool,
}

impl RecognitionEngine {
    pub fn new() -> Self {
        Self { is_listening: false }
    }

    pub fn start_realtime_recognition(&mut self) {
        self.is_listening = true;
    }

    pub fn process_talk_stream(&self, data: &[f32]) {
        if !data.is_empty() {
            println!("Recognizing talk stream: {} samples", data.len());
        }
    }
}
