pub struct PromptCore {
    pub history: Vec<String>,
}

impl PromptCore {
    pub fn load_local() -> Self {
        Self { 
            history: vec![
                "⚡ VELA PHANTOM STUDiO v1.1.0 — INITIALIZED".to_string(),
                "🕯️ BRAND BIO: Built on instinct, raw imagination, and algorithmic dualism.".to_string(),
                "📜 LORE ACTIVE: The legend of Vella Ville — where shadows trace the motion.".to_string(),
                "🎬 GENERATOR ENGINE: Ready for AI quotation, storyboard scripting, and frame rendering.".to_string()
            ] 
        }
    }

    pub fn submit_prompt(&mut self, prompt: &str) {
        let response = match prompt.to_lowercase().as_str() {
            "quote" => "“In twilight's glow, where shadows play, the phantom traces every unwritten line.” — VELA PHANTOM".to_string(),
            "storyboard" => "[SCENE 01]: Atmospheric sweep across Vella Ville. Neon pulse on wet asphalt. Camera tracks left as the black candle burns.".to_string(),
            "bio" => "VELA PHANTOM STUDiO: A dark-aesthetic cinematic and code framework engineered by KNOCKTURNALNC.".to_string(),
            _ => format!("> Executing generator sequence for: '{}'", prompt),
        };
        self.history.push(format!("Q: {}", prompt));
        self.history.push(response);
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }
}
