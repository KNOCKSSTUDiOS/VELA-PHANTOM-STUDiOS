pub struct RenderPipeline {
    pub resolution: (u32, u32),
}

impl Default for RenderPipeline {
    fn default() -> Self {
        Self { resolution: (1920, 1080) }
    }
}

impl RenderPipeline {
    pub fn compose_frame(&mut self, _frame: usize) {}
}
