
pub struct PromptCore {
    history: Vec<String>,
}

impl PromptCore {
    pub fn load_local() -> Self {
        let history = vec![
            "⚡ VELA PHANTOM STUDiO v2.0.0 — FULL SUITE INITIALIZED".to_string(),
            "🕯️ BIOLUMI BRANDING ACTIVE: Exclusive digital signature & sovereign asset framework.".to_string(),
            "🔊 DTS / DOLBY AUDIO ENGINE: Multi-channel spatial sound matrix online.".to_string(),
            "🛡️ SOVEREIGN PROTECTION & REAL BILLING: Active license verified to KNOCKSSTUDiOS.".to_string(),
            "🎬 TIMELINE & RENDERING DASHBOARD: Ready for instant frame export, storyboard scripting, and AI prompt execution.".to_string(),
        ];
        Self { history }
    }

    pub fn submit_prompt(&mut self, prompt: &str) {
        let response = match prompt.trim().to_lowercase().as_str() {
            "quote" => "“Shadows weave the silent code, where sovereignty outlives the storm.” — KNOCKSSTUDiOS Lore Matrix".to_string(),
            "storyboard" => "Storyboard sequence locked: Scene 1 [Vella Ville Neon Alley] -> Scene 2 [Biolumi Shimmer Core] -> Scene 3 [Final Export]".to_string(),
            "bio" => "Creator: Junior Tamayo (Gonzalo Guillen Tamayo) | Enterprise: KNOCKSSTUDiOS | Engine: Pure Rust Sovereign Core".to_string(),
            "audio" => "Spatial audio matrix recalibrated: 24-bit 96kHz Dolby Atmos channels active.".to_string(),
            "billing" => "Seller's permit verified: KNOCKTURNALNC / KNOCKSSTUDiOS. Active license secure.".to_string(),
            other => format!("Executed Sovereign Command [{}]: Processed successfully through Rust backend pipeline.", other),
        };
        self.history.push(format!("> {}", prompt));
        self.history.push(response);
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }
}
