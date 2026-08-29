// ============================================================================
// VELA PHANTOM STUDiO — MASTER ORCHESTRATOR & UI BINDING PIPELINE
// ============================================================================

use eframe::egui;

pub struct VelaPhantomStudioApp {
    pub timeline_state: timeline_engine::TimelineState,
    pub render_pipeline: render_engine::RenderPipeline,
    pub prompt_core: prompt_engine::PromptCore,
    pub prompt_input: String,
    pub is_rendering: bool,
    pub playback_speed: f32,
}

impl Default for VelaPhantomStudioApp {
    fn default() -> Self {
        Self {
            timeline_state: timeline_engine::TimelineState::new(),
            render_pipeline: render_engine::RenderPipeline::default(),
            prompt_core: prompt_engine::PromptCore::load_local(),
            prompt_input: String::new(),
            is_rendering: false,
            playback_speed: 1.0,
        }
    }
}

impl eframe::App for VelaPhantomStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Global Dark Atmospheric Styling (VELA PHANTOM Aesthetic)
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.window_fill = egui::Color32::from_rgb(15, 15, 20);
        style.visuals.panel_fill = egui::Color32::from_rgb(22, 22, 30);
        ctx.set_style(style);

        // Top Command & Menu Bar
        egui::TopBottomPanel::top("top_menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚡ VELA PHANTOM STUDiO");
                ui.separator();
                if ui.button("📁 New Project").clicked() {}
                if ui.button("💾 Save State").clicked() {}
                if ui.button("🚀 Seal App").clicked() {
                    self.is_rendering = true;
                }
            });
        });

        // Bottom AI Prompt & Control Box
        egui::TopBottomPanel::bottom("ai_prompt_panel").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("🤖 AI Prompt:");
                let prompt_response = ui.add(
                    egui::TextEdit::singleline(&mut self.prompt_input)
                        .hint_text("Enter generative prompt or query engine parameters...")
                        .desired_width(ui.available_width() - 100.0)
                );
                
                if ui.button("Generate").clicked() || (prompt_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    if !self.prompt_input.is_empty() {
                        self.prompt_core.submit_prompt(&self.prompt_input);
                        self.prompt_input.clear();
                    }
                }
            });
            ui.add_space(4.0);
        });

        // Central Timeline & Rendering Workspace
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                // Left Column: Viewport / Render Preview
                columns[0].vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(ui.available_width(), 350.0));
                        ui.centered_and_justified(|ui| {
                            if self.is_rendering {
                                ui.spinner();
                                ui.label("Rendering Frame Buffer...");
                            } else {
                                ui.label("📺 Render Viewport [Active]");
                            }
                        });
                    });

                    ui.add_space(10.0);
                    ui.heading("Engine Status");
                    ui.label("• timeline_engine: Synchronized");
                    ui.label("• render_engine: Locked");
                    ui.label("• prompt_engine: Ready");
                });

                // Right Column: AI Log & Output Feed
                columns[1].vertical(|ui| {
                    ui.heading("AI Generation Feed");
                    egio_scroll(ui, |ui| {
                        for entry in self.prompt_core.get_history() {
                            ui.label(format!("> {}", entry));
                        }
                    });
                });
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            // Timeline Scrubbing & Playback Controls
            ui.horizontal(|ui| {
                if ui.button("⏮").clicked() { self.timeline_state.seek_start(); }
                if ui.button(if self.timeline_state.is_playing() { "⏸" } else { "▶" }).clicked() {
                    self.timeline_state.toggle_playback();
                }
                if ui.button("⏭").clicked() { self.timeline_state.seek_end(); }

                ui.label(format!("Frame: {} / {}", self.timeline_state.current_frame(), self.timeline_state.total_frames()));
                
                let mut current_time = self.timeline_state.current_frame() as f32;
                let slider = ui.add(egui::Slider::new(&mut current_time, 0.0..=1000.0).text("Timeline Scrub"));
                if slider.changed() {
                    self.timeline_state.seek_frame(current_time as usize);
                }
            });
        });

        // Request continuous repaint for smooth UI timeline playback
        ctx.request_repaint();
    }
}

fn egio_scroll<F>(ui: &mut egui::Ui, add_contents: F) 
where
    F: FnOnce(&mut egui::Ui),
{
    egui::ScrollArea::vertical()
        .max_height(200.0)
        .auto_shrink([false; 2])
        .show(ui, add_contents);
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("VELA PHANTOM STUDiO v1.1.0"),
        ..Default::default()
    };
    eframe::run_native(
        "VELA PHANTOM STUDiO",
        options,
        Box::new(|_cc| Ok(Box::<VelaPhantomStudioApp>::default())),
    )
}
