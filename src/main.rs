use eframe::egui;
use prompt_engine::PromptCore;
use timeline_engine::TimelineState;
use render_engine::RenderCore;

#[derive(Default)]
struct VelaPhantomStudioApp {
    prompt_core: PromptCore,
    timeline_core: TimelineState,
    render_core: RenderCore,
    input_text: String,
    selected_tab: StudioTab,
    selected_palette: ColorPalette,
}

#[derive(PartialEq, Clone, Copy)]
enum StudioTab {
    Dashboard,
    MediaGallery,
    TimelineScrubber,
    RenderStudio,
    AudioSpatial,
}

impl Default for StudioTab {
    fn default() -> Self {
        StudioTab::Dashboard
    }
}

#[derive(PartialEq, Clone, Copy)]
enum ColorPalette {
    FresnoPeach,
    FlamingoPink,
    BioBlue,
    PearlEssentWhite,
    Gold,
}

impl Default for ColorPalette {
    fn default() -> Self {
        ColorPalette::PearlEssentWhite
    }
}

impl VelaPhantomStudioApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            prompt_core: PromptCore::load_local(),
            timeline_core: TimelineState::default(),
            render_core: RenderCore::default(),
            ..Default::default()
        }
    }

    fn apply_palette(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        
        let (bg_color, accent_color, text_color) = match self.selected_palette {
            ColorPalette::FresnoPeach => (
                egui::Color32::from_rgb(25, 12, 10),
                egui::Color32::from_rgb(255, 138, 101),
                egui::Color32::from_rgb(255, 224, 213),
            ),
            ColorPalette::FlamingoPink => (
                egui::Color32::from_rgb(20, 8, 15),
                egui::Color32::from_rgb(255, 105, 180),
                egui::Color32::from_rgb(255, 215, 235),
            ),
            ColorPalette::BioBlue => (
                egui::Color32::from_rgb(5, 15, 25),
                egui::Color32::from_rgb(0, 229, 255),
                egui::Color32::from_rgb(200, 245, 255),
            ),
            ColorPalette::PearlEssentWhite => (
                egui::Color32::from_rgb(12, 12, 14),
                egui::Color32::from_rgb(240, 240, 245),
                egui::Color32::from_rgb(220, 220, 230),
            ),
            ColorPalette::Gold => (
                egui::Color32::from_rgb(15, 12, 5),
                egui::Color32::from_rgb(255, 215, 0),
                egui::Color32::from_rgb(255, 248, 220),
            ),
        };

        style.visuals.dark_mode = true;
        style.visuals.window_fill = bg_color;
        style.visuals.panel_fill = bg_color;
        style.visuals.selection.bg_fill = accent_color;
        style.visuals.selection.stroke = egui::Stroke::new(1.0_f32, text_color);
        
        ctx.set_style(style);
    }
}

impl eframe::App for VelaPhantomStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_palette(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚡ VELA PHANTOM STUDiO");
                ui.separator();
                if ui.button("Dashboard").clicked() { self.selected_tab = StudioTab::Dashboard; }
                if ui.button("Media Gallery").clicked() { self.selected_tab = StudioTab::MediaGallery; }
                if ui.button("Timeline").clicked() { self.selected_tab = StudioTab::TimelineScrubber; }
                if ui.button("Render 2D/3D").clicked() { self.selected_tab = StudioTab::RenderStudio; }
                if ui.button("Spatial Sound").clicked() { self.selected_tab = StudioTab::AudioSpatial; }
                
                ui.separator();
                ui.label("Palette:");
                if ui.selectable_label(self.selected_palette == ColorPalette::FresnoPeach, "Fresno Peach").clicked() { self.selected_palette = ColorPalette::FresnoPeach; }
                if ui.selectable_label(self.selected_palette == ColorPalette::FlamingoPink, "Flamingo Pink").clicked() { self.selected_palette = ColorPalette::FlamingoPink; }
                if ui.selectable_label(self.selected_palette == ColorPalette::BioBlue, "Bio Blue").clicked() { self.selected_palette = ColorPalette::BioBlue; }
                if ui.selectable_label(self.selected_palette == ColorPalette::PearlEssentWhite, "Pearl White").clicked() { self.selected_palette = ColorPalette::PearlEssentWhite; }
                if ui.selectable_label(self.selected_palette == ColorPalette::Gold, "Gold").clicked() { self.selected_palette = ColorPalette::Gold; }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.selected_tab {
                StudioTab::Dashboard => {
                    ui.heading("Sovereignty & Prompt Control Matrix");
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        ui.label("Prompt Input:");
                        ui.text_edit_singleline(&mut self.input_text);
                        if ui.button("Submit").clicked() && !self.input_text.is_empty() {
                            let prompt = self.input_text.clone();
                            self.prompt_core.submit_prompt(&prompt);
                            self.input_text.clear();
                        }
                    });

                    ui.add_space(10.5);
                    ui.label("Execution Feed & Lore History:");
                    egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                        for line in self.prompt_core.get_history().iter().rev() {
                            ui.label(line);
                        }
                    });
                }
                StudioTab::MediaGallery => {
                    ui.heading("Media Gallery (PNG, JPG, MP3, MP4, 2D / 3D Asset Previews)");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.group(|ui| {
                            ui.set_min_size(egui::vec2(180.0, 120.0));
                            ui.label("📁 [3D MESH ASSET 01]");
                            ui.label("Type: .obj / .fbx (Polygon Matrix)");
                            ui.label("Status: Indexed & Ready");
                        });
                        ui.group(|ui| {
                            ui.set_min_size(egui::vec2(180.0, 120.0));
                            ui.label("📁 [2D RENDER PREVIEW]");
                            ui.label("Type: .png / .jpg (High Res)");
                            ui.label("Status: Indexed & Ready");
                        });
                        ui.group(|ui| {
                            ui.set_min_size(egui::vec2(180.0, 120.0));
                            ui.label("📁 [AUDIO STREAM]");
                            ui.label("Type: .mp3 / .wav (Spatial)");
                            ui.label("Status: Indexed & Ready");
                        });
                        ui.group(|ui| {
                            ui.set_min_size(egui::vec2(180.0, 120.0));
                            ui.label("📁 [VIDEO SEQUENCE]");
                            ui.label("Type: .mp4 (4K Stream)");
                            ui.label("Status: Indexed & Ready");
                        });
                    });
                }
                StudioTab::TimelineScrubber => {
                    ui.heading("Multi-Track Timeline Scrubber");
                    ui.separator();
                    ui.label("Scrubbing frame position across tracks:");
                    if ui.button("Step Forward Frame").clicked() {
                        self.timeline_core.step();
                    }
                    ui.label(format!("Current Frame Marker: {}", self.timeline_core.get_frame()));
                }
                StudioTab::RenderStudio => {
                    ui.heading("2D & 3D Render Engine");
                    ui.separator();
                    ui.label("Select Pipeline Mode:");
                    if ui.button("Export 2D Vector Frame").clicked() {
                        self.render_core.trigger_2d();
                    }
                    if ui.button("Compile 3D Volumetric Mesh").clicked() {
                        self.render_core.trigger_3d();
                    }
                    ui.label(format!("Render Log: {}", self.render_core.get_status()));
                }
                StudioTab::AudioSpatial => {
                    ui.heading("DTS & Dolby Spatial Sound Matrix");
                    ui.separator();
                    ui.label("Multi-channel audio telemetry locked at 24-bit 96kHz sovereign output.");
                    if ui.button("Initialize Spatial Sound Matrix").clicked() {
                        // Spatial audio hook trigger
                    }
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Vela Phantom Studio",
        options,
        Box::new(|_cc| Ok(Box::<VelaPhantomStudioApp>::default())),
    )
}
