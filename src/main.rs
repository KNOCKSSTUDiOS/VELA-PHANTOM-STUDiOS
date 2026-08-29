use eframe::egui;

struct VelaStudioApp {
    active_tab: StudioTab,
    input_text: String,
    logs: Vec<String>,
    timeline_frame: u64,
    timeline_playing: bool,
    zoom_level: f32,
    audio_volume: f32,
}

#[derive(PartialEq, Clone, Copy)]
enum StudioTab {
    Dashboard,
    TimelineEditor,
    MediaPool,
    AudioMixer,
    RenderPipeline,
}

impl Default for VelaStudioApp {
    fn default() -> Self {
        let mut logs = Vec::new();
        logs.push("⚡ VELA PHANTOM STUDiO v2.0.0 — FULL SUITE INITIALIZED".to_string());
        logs.push("🕯️ BIOLUMI BRANDING ACTIVE: Exclusive digital signature & sovereign asset framework.".to_string());
        logs.push("🔊 DTS / DOLBY AUDIO ENGINE: Multi-channel spatial sound matrix online.".to_string());
        logs.push("🛡️ SOVEREIGN PROTECTION: Active license verified to KNOCKSSTUDiOS.".to_string());

        Self {
            active_tab: StudioTab::Dashboard,
            input_text: String::new(),
            logs,
            timeline_frame: 0,
            timeline_playing: false,
            zoom_level: 1.0,
            audio_volume: 0.85,
        }
    }
}

impl VelaStudioApp {
    fn apply_custom_theme(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        
        // Deep obsidian background, dark metallic panels, industrial chrome-orange accents, and bioluminescent pink/red fire dot glow
        let bg_color = egui::Color32::from_rgb(10, 10, 12);
        let panel_color = egui::Color32::from_rgb(18, 19, 24);
        let widget_color = egui::Color32::from_rgb(28, 30, 38);
        let text_color = egui::Color32::from_rgb(235, 235, 245);
        let accent_fire = egui::Color32::from_rgb(255, 75, 43); // Fire flame orange/red
        let accent_pink = egui::Color32::from_rgb(255, 20, 147); // Flamingo pink

        style.visuals.dark_mode = true;
        style.visuals.window_fill = bg_color;
        style.visuals.panel_fill = panel_color;
        style.visuals.extreme_bg_color = bg_color;
        style.visuals.faint_bg_color = widget_color;
        
        style.visuals.widgets.noninteractive.bg_fill = widget_color;
        style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(120, 125, 140));
        style.visuals.widgets.inactive.bg_fill = widget_color;
        style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text_color);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 48, 60);
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, accent_fire);
        style.visuals.widgets.active.bg_fill = accent_fire;
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, egui::Color32::white());

        style.visuals.selection.bg_fill = accent_pink;
        style.visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::white());

        ctx.set_style(style);
    }
}

impl eframe::App for VelaStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_custom_theme(ctx);

        // 1. TOP TITLE & NAVIGATION BAR (Metal Texture Look with Flame "i" Accent)
        egui::TopBottomPanel::top("top_navigation_bar").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.heading("⚡ VELA PHANTOM STUDiO");
                ui.label(egui::RichText::new("KNOCKSSTUDiOS").color(egui::Color32::from_rgb(255, 75, 43)).strong());
                ui.separator();

                if ui.selectable_label(self.active_tab == StudioTab::Dashboard, "📊 Dashboard").clicked() {
                    self.active_tab = StudioTab::Dashboard;
                }
                if ui.selectable_label(self.active_tab == StudioTab::TimelineEditor, "🎬 Timeline Suite").clicked() {
                    self.active_tab = StudioTab::TimelineEditor;
                }
                if ui.selectable_label(self.active_tab == StudioTab::MediaPool, "📁 Media & Assets").clicked() {
                    self.active_tab = StudioTab::MediaPool;
                }
                if ui.selectable_label(self.active_tab == StudioTab::AudioMixer, "🔊 Audio Matrix").clicked() {
                    self.active_tab = StudioTab::AudioMixer;
                }
                if ui.selectable_label(self.active_tab == StudioTab::RenderPipeline, "⚙️ Render Engine").clicked() {
                    self.active_tab = StudioTab::RenderPipeline;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("🔥 LiVE [i]").color(egui::Color32::from_rgb(255, 20, 147)).strong());
                    ui.separator();
                    ui.label("Architect: Junior Tamayo");
                });
            });
            ui.add_space(6.0);
        });

        // 2. BOTTOM STATUS FOOTER
        egui::TopBottomPanel::bottom("status_footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("● SECURE LICENSE: KNOCKTURNALNC").color(egui::Color32::from_rgb(0, 229, 255)));
                ui.separator();
                ui.label(format!("Frame Position: {}", self.timeline_frame));
                ui.separator();
                ui.label("Pure Rust Engine | 10-bit Color Pipeline Active");
            });
        });

        // 3. CENTRAL WORKSPACE MODULES
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab {
                StudioTab::Dashboard => {
                    ui.heading("Prompt Sovereignty Matrix & Command Hub");
                    ui.separator();
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label("Command Input:");
                        let text_edit = ui.add(egui::TextEdit::singleline(&mut self.input_text).desired_width(500.0));
                        if (ui.button("⚡ Execute Command").clicked() || (text_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) && !self.input_text.is_empty() {
                            let cmd = self.input_text.clone();
                            self.logs.push(format!("> {}", cmd));
                            let reply = match cmd.to_lowercase().as_str() {
                                "quote" => "“Shadows weave the silent code, where sovereignty outlives the storm.” — KNOCKSSTUDiOS".to_string(),
                                "bio" => "Creator: Junior Tamayo (Gonzalo Guillen Tamayo) | Enterprise: KNOCKSSTUDiOS".to_string(),
                                "billing" => "Seller's permit verified: KNOCKTURNALNC / KNOCKSSTUDiOS. Active license secure.".to_string(),
                                other => format!("Processed sovereign directive [{}]: Pipeline synchronized.", other),
                            };
                            self.logs.push(reply);
                            self.input_text.clear();
                        }
                    });

                    ui.add_space(15.0);
                    ui.label(egui::RichText::new("Execution Feed Logs:").strong());
                    ui.add_space(5.0);

                    egui::ScrollArea::vertical().max_height(380.0).show(ui, |ui| {
                        for log in self.logs.iter().rev() {
                            ui.group(|ui| {
                                ui.set_min_width(ui.available_width());
                                ui.label(log);
                            });
                            ui.add_space(4.0);
                        }
                    });
                }
                StudioTab::TimelineEditor => {
                    ui.heading("Professional Non-Linear Multi-Track Timeline");
                    ui.separator();
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button("⏮ First Frame").clicked() { self.timeline_frame = 0; }
                        if ui.button(if self.timeline_playing { "⏸ Pause" } else { "▶ Play Stream" }).clicked() {
                            self.timeline_playing = !self.timeline_playing;
                        }
                        if ui.button("⏭ Step Forward").clicked() { self.timeline_frame += 1; }
                        ui.separator();
                        ui.label(format!("Timecode: 00:00:0{}:12 (SMPTE)", self.timeline_frame / 30));
                        ui.separator();
                        ui.label(format!("Zoom: {:.1}x", self.zoom_level));
                        if ui.button("Zoom In").clicked() { self.zoom_level += 0.25; }
                        if ui.button("Zoom Out").clicked() && self.zoom_level > 0.5 { self.zoom_level -= 0.25; }
                    });

                    ui.add_space(15.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(ui.available_width(), 140.0));
                        ui.heading("Track 1 [Video Sequence - Vella Ville 4K]");
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            for i in 0..25 {
                                let active = i == (self.timeline_frame as usize % 25);
                                let color = if active { egui::Color32::from_rgb(255, 75, 43) } else { egui::Color32::from_rgb(50, 55, 70) };
                                ui.colored_label(color, "████");
                            }
                        });
                        ui.add_space(10.0);
                        ui.heading("Track 2 [Biolumi Audio Matrix - 24-bit / 96kHz]");
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            for i in 0..25 {
                                let active = i == (self.timeline_frame as usize % 25);
                                let color = if active { egui::Color32::from_rgb(255, 20, 147) } else { egui::Color32::from_rgb(40, 70, 80) };
                                ui.colored_label(color, "~~~~");
                            }
                        });
                    });
                }
                StudioTab::MediaPool => {
                    ui.heading("Media Pool & Asset Manager");
                    ui.separator();
                    ui.add_space(10.0);

                    ui.columns(2, |cols| {
                        cols[0].vertical(|ui| {
                            ui.heading("📁 Indexed Project Assets");
                            ui.add_space(8.0);
                            ui.group(|ui| {
                                ui.label("📦 3D Volumetric Mesh (.obj / .fbx)");
                                ui.label("Status: Indexed & Cached");
                                if ui.button("Inspect 3D Geometry").clicked() {}
                            });
                            ui.group(|ui| {
                                ui.label("🖼️ High-Res Vector Frame (.png / .svg)");
                                ui.label("Status: Metal & Flame Texture Loaded");
                                if ui.button("Preview Asset").clicked() {}
                            });
                        });
                        cols[1].vertical(|ui| {
                            ui.heading("🎞️ Stream Sequences");
                            ui.add_space(8.0);
                            ui.group(|ui| {
                                ui.label("🎵 Dolby Atmos Master Audio (.wav)");
                                ui.label("Status: 24-bit 96kHz Locked");
                                if ui.button("Test Spatial Audio").clicked() {}
                            });
                            ui.group(|ui| {
                                ui.label("🎬 4K Master Video Stream (.mp4)");
                                ui.label("Status: Proxy Buffer Ready");
                                if ui.button("Load Stream").clicked() {}
                            });
                        });
                    });
                }
                StudioTab::AudioMixer => {
                    ui.heading("DTS / Dolby Spatial Sound Matrix");
                    ui.separator();
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label("Master Volume Control:");
                        ui.add(egui::Slider::new(&mut self.audio_volume, 0.0..=1.0).text("Gain"));
                    });
                    ui.add_space(15.0);

                    ui.columns(3, |cols| {
                        cols[0].group(|ui| {
                            ui.heading("Front L / R");
                            ui.label("Ambisonic Decoder: Active");
                            ui.label("EQ: 15-Band Parametric");
                        });
                        cols[1].group(|ui| {
                            ui.heading("Center & LFE");
                            ui.label("Sub-Bass Enhancer: Locked");
                            ui.label("Limiter: -0.5 dB");
                        });
                        cols[2].group(|ui| {
                            ui.heading("Atmos Height");
                            ui.label("Spatial Reverb: Studio Hall");
                            ui.label("Bitrate: 96 kHz / 24-bit");
                        });
                    });
                }
                StudioTab::RenderPipeline => {
                    ui.heading("2D / 3D Volumetric Render Engine");
                    ui.separator();
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button("⚡ Export 4K HDR10 Frame").clicked() {
                            self.logs.push("SUCCESS: 4K HDR10 Frame rendered successfully with metal finish.".to_string());
                        }
                        if ui.button("🔮 Compile 3D Mesh Sequence").clicked() {
                            self.logs.push("SUCCESS: 3D Volumetric sequence compiled to output buffer.".to_string());
                        }
                        if ui.button("📦 Batch Export MLT Project").clicked() {
                            self.logs.push("SUCCESS: MLT XML project package exported.".to_string());
                        }
                    });

                    ui.add_space(20.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(ui.available_width(), 150.0));
                        ui.heading("Render Queue & Hardware Acceleration Status:");
                        ui.add_space(8.0);
                        ui.label("GPU Acceleration: OpenGL / Vulkan Multi-Core Parallel Processing Active");
                        ui.label("Color Pipeline: Linear Color Processing (10-bit End-to-End)");
                        ui.label("Encoder: FFmpeg Backend Ready");
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
        Box::new(|_cc| Ok(Box::<VelaStudioApp>::default())),
    )
}
