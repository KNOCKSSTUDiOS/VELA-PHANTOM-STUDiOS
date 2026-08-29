pub struct PromptCore {
    pub history: Vec<String>,
}

impl PromptCore {
    pub fn load_local() -> Self {
        Self { history: vec!["VELA PHANTOM STUDiO v1.1.0 Initialized".to_string()] }
    }
    pub fn submit_prompt(&mut self, prompt: &str) {
        self.history.push(prompt.to_string());
    }
    pub fn get_history(&self) -> &[String] {
        &self.history
    }
}
