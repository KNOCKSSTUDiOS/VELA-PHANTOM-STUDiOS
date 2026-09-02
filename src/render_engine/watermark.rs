pub struct WatermarkController {
    pub opacity: f32,
    pub position: WatermarkPosition,
    pub show_monogram: bool,
}

#[derive(Debug)]
pub enum WatermarkPosition { BottomRight, TopRight, TopLeft, BottomLeft, Center }

impl WatermarkController {
    pub fn new() -> Self {
        Self { opacity: 0.15, position: WatermarkPosition::BottomRight, show_monogram: true }
    }
    pub fn render_overlay(&self) {
        println!("Rendering VELA INFINITY-CORE Watermark at {:?} with {}% opacity",
            self.position, (self.opacity * 100.0) as u32);
    }
}
