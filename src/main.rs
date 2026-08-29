use eframe::egui;

use sound_engine::run_sound_engine;
use scene_engine::run_scene_engine;
use render_engine::run_render_engine;
use prompt_engine::run_prompt_engine;
use timeline_engine::run_timeline_engine;
use physics_engine::run_physics_engine;
use camera_engine::run_camera_engine;
use studio_engine::run_studio_engine;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "VELA PHANTOM STUDiO",
        options,
        Box::new(|_cc| Box::new(StudioApp::default())),
    )
}

#[derive(Default)]
struct StudioApp {
    log: String,
    splash_done: bool,
    splash_timer: f32,
}

impl eframe::App for StudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.unstable_dt);

        if !self.splash_done {
            self.splash_timer += dt;

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(80.0);
                    ui.heading("VELA PHANTOM STUDiO");
                    ui.label("Booting studio engine...");
                    ui.label("Loading GUI...");
                    ui.label("Initializing timeline...");
                    ui.label("Preparing render pipeline...");
                });
            });

            if self.splash_timer >= 2.0 {
                self.splash_done = true;
            }

            ctx.request_repaint();
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("VELA PHANTOM STUDiO");

            if ui.button("Run Full Pipeline").clicked() {
                self.log.push_str("Sound engine running.\n");
                run_sound_engine();

                self.log.push_str("Scene engine running.\n");
                run_scene_engine();

                self.log.push_str("Render engine running.\n");
                run_render_engine();

                self.log.push_str("Prompt engine running.\n");
                run_prompt_engine();

                self.log.push_str("Timeline engine running.\n");
                run_timeline_engine();

                self.log.push_str("Physics engine running.\n");
                run_physics_engine();

                self.log.push_str("Camera engine running.\n");
                run_camera_engine();

                self.log.push_str("Studio engine running.\n");
                run_studio_engine();

                self.log.push_str("Pipeline complete.\n");
            }

            ui.separator();
            ui.label(&self.log);
        });
    }
}
