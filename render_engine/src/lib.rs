#[derive(Default, Clone)]
pub struct RenderCore {
    pub status: String,
}

impl RenderCore {
    pub fn trigger_2d(&mut self) {
        self.status = "2D Vector Frame Exported Successfully.".to_string();
    }

    pub fn trigger_3d(&mut self) {
        self.status = "3D Volumetric Mesh Compiled Successfully.".to_string();
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}
