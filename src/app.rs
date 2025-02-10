use eframe::{get_value, set_value, App, CreationContext, Frame, Storage, APP_KEY};
use egui::{menu, widgets, CentralPanel, Context, TopBottomPanel, ViewportCommand, Window};

use crate::components::canvas::Canvas;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct P2e {
    canvas: Canvas,
}

impl Default for P2e {
    fn default() -> Self {
        Self {
            canvas: Canvas::new([512, 512]),
        }
    }
}

impl P2e {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl App for P2e {
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                widgets::global_theme_preference_buttons(ui);
            });
        });

        CentralPanel::default().show(ctx, |_ui| {
            Window::new("Input")
                .default_size([512., 512.])
                .resizable(false)
                .show(ctx, |ui| {
                    self.canvas.update(ctx, ui);
                });
        });
    }
}
