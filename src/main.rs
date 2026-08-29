use eframe::egui;
use prompt_engine::PromptCore;
use timeline_engine::TimelineState;
use render_engine::RenderCore;

struct VelaApp {
    prompt_core: PromptCore,
    timeline_core: TimelineState,
    render_core: RenderCore,
    input_text: String,
    active_menu: MenuTab,
    theme: ThemeStyle,
}

#[derive(PartialEq, Clone, Copy)]
enum MenuTab {
    Dashboard,
    MediaPool,
    TimelineEditor,
    RenderPipeline,
    AudioMixer,
}

#[derive(PartialEq, Clone, Copy)]
enum ThemeStyle {
    DarkKnight,
    CyberpunkPink,
    BiolumiBlue,
    FresnoPeach,
    GoldLuxury,
}

impl Default for VelaApp {
    fn default() -> Self {
        Self {
            prompt_core: PromptCore::load_local(),
            timeline_core: TimelineState::default(),
            render_core: RenderCore::default(),
            input_text: String::new(),
            active_menu: MenuTab::Dashboard,
            theme: ThemeStyle::DarkKnight,
        }
    }
}

impl VelaApp {
    fn apply_theme(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        let (bg, panel, accent, text) = match self.theme {
            ThemeStyle::DarkKnight => (
                egui::Color32::from_rgb(15, 15, 18),
                egui::Color32::from_rgb(24, 24, 28),
                egui::Color32::from_rgb(220, 220, 230),
                egui::Color32::from_rgb(240, 240, 250),
            ),
            ThemeStyle::CyberpunkPink => (
                egui::Color32::from_rgb(20, 8, 15),
                egui::Color32::from_rgb(35, 12, 25),
                egui::Color32::from_rgb(255, 105, 180),
                egui::Color32::from_rgb(255, 215, 235),
            ),
            ThemeStyle::BiolumiBlue => (
                egui::Color32::from_rgb(5, 15, 25),
                egui::Color32::from_rgb(10, 28, 45),
                egui::Color32::from_rgb(0, 229, 255),
                egui::Color32::from_rgb(200, 245, 255),
            ),
            ThemeStyle::FresnoPeach => (
                egui::Color32::from_rgb(25, 12, 10),
                egui::Color32::from_rgb(45, 20, 15),
                egui::Color32::from_rgb(255, 138, 101),
                egui::Color32::from_rgb(255, 224, 213),
            ),
            ThemeStyle::GoldLuxury => (
                egui::Color32::from_rgb(15, 12, 5),
                egui::Color32::from_rgb(30, 24, 10),
                egui::Color32::from_rgb(255, 215, 0),
                egui::Color32::from_rgb(255, 248, 220),
            ),
        };

        style.visuals.dark_mode = true;
        style.visuals.window_fill = bg;
        style.visuals.panel_fill = panel;
        style.visuals.selection.bg_fill = accent;
        style.visuals.selection.stroke = egui::Stroke::new(1.0, text);
        ctx.set_style(style);
    }
}

impl eframe::App for VelaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme(ctx);

        // Top Navigation Bar (Full native app menu header)
        egui::TopBottomPanel::top("top_menu_bar").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.heading("⚡ VELA PHANTOM STUDiO");
                ui.separator();

                if ui.selectable_label(self.active_menu == MenuTab::Dashboard, "📊 Dashboard").clicked() {
                    self.active_menu = MenuTab::Dashboard;
                }
                if ui.selectable_label(self.active_menu == MenuTab::MediaPool, "📁 Media Pool").clicked() {
                    self.active_menu = MenuTab::MediaPool;
                }
                if ui.selectable_label(self.active_menu == MenuTab::TimelineEditor, "🎬 Timeline").clicked() {
                    self.active_menu = MenuTab::TimelineEditor;
                }
                if ui.selectable_label(self.active_menu == MenuTab::RenderPipeline, "⚙️ Render Engine").clicked() {
                    self.active_menu = MenuTab::RenderPipeline;
                }
                if ui.selectable_label(self.active_menu == MenuTab::AudioMixer, "🔊 Audio Suite").clicked() {
                    self.active_menu = MenuTab::AudioMixer;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button("🎨 Theme Style", |ui| {
                        if ui.button("Dark Knight").clicked() { self.theme = ThemeStyle::DarkKnight; ui.close_menu(); }
                        if ui.button("Cyberpunk Pink").clicked() { self.theme = ThemeStyle::CyberpunkPink; ui.close_menu(); }
                        if ui.button("Biolumi Blue").clicked() { self.theme = ThemeStyle::BiolumiBlue; ui.close_menu(); }
                        if ui.button("Fresno Peach").clicked() { self.theme = ThemeStyle::FresnoPeach; ui.close_menu(); }
                        if ui.button("Gold Luxury").clicked() { self.theme = ThemeStyle::GoldLuxury; ui.close_menu(); }
                    });
                    ui.label("KNOCKSSTUDiOS 🔒");
                });
            });
            ui.add_space(4.0);
        });

        // Bottom Status Footer
        egui::TopBottomPanel::bottom("status_footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status: Active Sovereign License Locked");
                ui.separator();
                ui.label(format!("Current Frame: {}", self.timeline_core.get_frame()));
                ui.separator();
                ui.label("Engine: Pure Rust Native Build");
            });
        });

        // Central Main Workspace Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_menu {
                MenuTab::Dashboard => {
                    ui.heading("Prompt Sovereignty Matrix & Execution Feed");
                    ui.separator();
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label("Prompt Command:");
                        let response = ui.add(egui::TextEdit::singleline(&mut self.input_text).desired_width(400.0));
                        if (ui.button("Submit Prompt").clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) && !self.input_text.is_empty() {
                            let prompt = self.input_text.clone();
                            self.prompt_core.submit_prompt(&prompt);
                            self.input_text.clear();
                        }
                    });

                    ui.add_space(12.0);
                    ui.label("Execution History Log:");
                    egui::ScrollArea::vertical().max_height(350.0).show(ui, |ui| {
                        for line in self.prompt_core.get_history().iter().rev() {
                            ui.group(|ui| {
                                ui.set_min_width(ui.available_width());
                                ui.label(line);
                            });
                            ui.add_space(4.0);
                        }
                    });
                }
                MenuTab::MediaPool => {
                    ui.heading("Project Media Pool & Asset Browser");
                    ui.separator();
                    ui.add_space(8.0);

                    ui.columns(2, |columns| {
                        columns[0].vertical(|ui| {
                            ui.heading("Indexed Assets");
                            ui.add_space(6.0);
                            ui.group(|ui| {
                                ui.label("📦 3D Mesh Asset 01 (.obj / .fbx)");
                                ui.label("Status: Volumetric Matrix Indexed");
                                if ui.button("Inspect Mesh").clicked() {}
                            });
                            ui.group(|ui| {
                                ui.label("🖼️ 2D Vector Frame (.png / .jpg)");
                                ui.label("Status: High-Res Buffer Ready");
                                if ui.button("Preview Frame").clicked() {}
                            });
                        });
                        columns[1].vertical(|ui| {
                            ui.heading("Streams & Audio");
                            ui.add_space(6.0);
                            ui.group(|ui| {
                                ui.label("🎵 Audio Stream (.mp3 / .wav)");
                                ui.label("Status: 24-bit 96kHz Spatial Ready");
                                if ui.button("Test Playback").clicked() {}
                            });
                            ui.group(|ui| {
                                ui.label("🎞️ Video Sequence (.mp4)");
                                ui.label("Status: 4K Stream Buffer Allocated");
                                if ui.button("Load Sequence").clicked() {}
                            });
                        });
                    });
                }
                MenuTab::TimelineEditor => {
                    ui.heading("Multi-Track Timeline & Frame Scrubber");
                    ui.separator();
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui.button("⏮️ Jump to Start").clicked() {}
                        if ui.button("▶ Step Frame Forward").clicked() {
                            self.timeline_core.step();
                        }
                        if ui.button("⏸ Pause").clicked() {}
                        ui.label(format!("Active Frame Position: {}", self.timeline_core.get_frame()));
                    });

                    ui.add_space(20.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(ui.available_width(), 120.0));
                        ui.label("Timeline Track Visualizer:");
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            for i in 0..15 {
                                let color = if i == (self.timeline_core.get_frame() as usize % 15) {
                                    egui::Color32::from_rgb(0, 229, 255)
                                } else {
                                    egui::Color32::from_rgb(60, 60, 70)
                                };
                                ui.colored_label(color, "█");
                            }
                        });
                    });
                }
                MenuTab::RenderPipeline => {
                    ui.heading("2D / 3D Volumetric Render Engine");
                    ui.separator();
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui.button("⚡ Export 2D Vector Frame").clicked() {
                            self.render_core.trigger_2d();
                        }
                        if ui.button("🔮 Compile 3D Volumetric Mesh").clicked() {
                            self.render_core.trigger_3d();
                        }
                    });

                    ui.add_space(15.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(ui.available_width(), 100.0));
                        ui.heading("Pipeline Log Output:");
                        ui.label(self.render_core.get_status());
                    });
                }
                MenuTab::AudioMixer => {
                    ui.heading("Spatial Sound & Audio Matrix");
                    ui.separator();
                    ui.add_space(8.0);

                    ui.label("DTS & Dolby Spatial Sound Channels (24-bit / 96kHz)");
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button("Initialize Spatial Matrix").clicked() {}
                        if ui.button("Calibrate Channels").clicked() {}
                    });

                    ui.add_space(15.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(ui.available_width(), 100.0));
                        ui.label("Channel 01: Front Left [Active]");
                        ui.label("Channel 02: Front Right [Active]");
                        ui.label("Channel 03: Dolby Atmos Height [Locked]");
                    });
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
        Box::new(|_cc| Ok(Box::<VelaApp>::default())),
    )
}
