use egui::{Color32, Visuals};

pub struct VelaTheme;

impl VelaTheme {
    pub const FRESNO_PEACH: Color32 = Color32::from_rgb(255, 179, 102);
    pub const FLAMINGO_PINK: Color32 = Color32::from_rgb(255, 75, 145);
    pub const CANVAS_WHITE: Color32 = Color32::from_rgb(245, 245, 245);
    
    pub fn visuals() -> Visuals {
        let mut visuals = Visuals::dark();
        visuals.override_text_color = Some(Self::CANVAS_WHITE);
        visuals.selection.bg_fill = Self::FLAMINGO_PINK;
        visuals.selection.stroke.color = Self::FRESNO_PEACH;
        visuals.widgets.active.bg_fill = Self::FLAMINGO_PINK;
        visuals.widgets.hovered.bg_fill = Self::FRESNO_PEACH;
        visuals
    }
}
