use eframe::App;
use egui::Window;

use crate::components::canvas::Canvas;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct P2e {
    canvas: Canvas,
}

impl Default for P2e {
    fn default() -> Self {
        Self {
            canvas: Canvas::new(),
        }
    }
}

impl P2e {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl App for P2e {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            Window::new("Canvas")
                .default_size([512., 512.])
                .resizable(false)
                .show(ctx, |ui| {
                    self.canvas.update(ctx, ui);
                });
        });
    }
}
