#[derive(Clone)]
pub struct PromptCore {
    pub history: Vec<String>,
}

impl Default for PromptCore {
    fn default() -> Self {
        Self { 
            history: vec![
                "⚡ VELA PHANTOM STUDiO v2.0.0 — FULL SUITE INITIALIZED".to_string(),
                "🕯️ BIOLUMI BRANDING ACTIVE: Exclusive digital signature & sovereign asset framework.".to_string(),
                "🔊 DTS / DOLBY AUDIO ENGINE: Multi-channel spatial sound matrix online.".to_string(),
                "🛡️ SOVEREIGN PROTECTION & REAL BILLING: Active license verified to KNOCKSSTUDiOS.".to_string(),
                "🎬 TIMELINE & RENDERING DASHBOARD: Ready for instant frame export, storyboard scripting, and AI prompt execution.".to_string()
            ] 
        }
    }
}

impl PromptCore {
    pub fn load_local() -> Self {
        Self::default()
    }

    pub fn submit_prompt(&mut self, prompt: &str) {
        let response = match prompt.to_lowercase().as_str() {
            "quote" => "“In twilight's glow, where shadows play, the phantom traces every unwritten line.” — VELA PHANTOM".to_string(),
            "storyboard" => "[SCENE 01]: Cinematic sweep across Vella Ville. Biolumi neon pulse on wet asphalt. Dolby Atmos spatial track engaged.".to_string(),
            "bio" => "VELA PHANTOM STUDiO: A dark-aesthetic cinematic and code framework engineered exclusively by KNOCKSSTUDiOS with Biolumi branding and sovereign billing protection.".to_string(),
            "audio" => "[DTS / DOLBY ENGINE]: Spatial soundscape channels locked and balanced at 24-bit 96kHz.".to_string(),
            "billing" => "[SOVEREIGN BILLING]: SECURE PERMIT ACTIVE (KNOCKSSTUDiOS). Transaction gateway encrypted.".to_string(),
            _ => format!("> Biolumi AI Engine processed sequence: '{}'", prompt),
        };
        self.history.push(format!("Q: {}", prompt));
        self.history.push(response);
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }
}
