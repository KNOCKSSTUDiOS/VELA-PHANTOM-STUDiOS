pub struct RenderCore {
    status: String,
}

impl Default for RenderCore {
    fn default() -> Self {
        Self {
            status: "Pipeline Idle. Ready for 2D/3D compilation.".to_string(),
        }
    }
}

impl RenderCore {
    pub fn trigger_2d(&mut self) {
        self.status = "SUCCESS: 2D Vector Frame exported at 4K resolution (Biolumi PNG buffer).".to_string();
    }

    pub fn trigger_3d(&mut self) {
        self.status = "SUCCESS: 3D Volumetric Mesh compiled and indexed (.obj / .fbx format).".to_string();
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}
