use egui::{Color32, Visuals};

pub const COLOR_FLUXLINE_BLUE: Color32 = Color32::from_rgb(0, 119, 255);
pub const COLOR_EMBERSTONE_ORANGE: Color32 = Color32::from_rgb(255, 106, 0);
pub const COLOR_BIO_CYAN: Color32 = Color32::from_rgb(0, 229, 255);
pub const COLOR_MANGO_GOLD: Color32 = Color32::from_rgb(212, 175, 55);
pub const COLOR_OBSIDIAN_VOID: Color32 = Color32::from_rgb(5, 6, 10);
pub const COLOR_WHITE_PURE: Color32 = Color32::from_rgb(245, 245, 245);

pub fn apply_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    visuals.panel_fill = COLOR_OBSIDIAN_VOID;
    visuals.window_fill = COLOR_OBSIDIAN_VOID;
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, COLOR_BIO_CYAN.gamma_multiply(0.15));
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, COLOR_WHITE_PURE.gamma_multiply(0.6));
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(2.0, COLOR_BIO_CYAN);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, COLOR_EMBERSTONE_ORANGE);
    ctx.set_visuals(visuals);
}
