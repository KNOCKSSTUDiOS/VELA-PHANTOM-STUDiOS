pub struct PromptEngine {
    pub history: Vec<String>,
}

impl PromptEngine {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn generate_intelligence(&mut self, prompt: &str) -> String {
        self.history.push(prompt.to_string());
        format!("VELA_PARAM_GEN: {}", prompt)
    }
}
