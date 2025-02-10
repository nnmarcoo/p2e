use eframe::{get_value, set_value, App, CreationContext, Frame, Storage, APP_KEY};
use egui::{CentralPanel, Context, Window};

use crate::components::{canvas::Canvas, menu_bar::MenuBar};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct P2e {
    canvas: Canvas,
    menu_bar: MenuBar,
}

impl Default for P2e {
    fn default() -> Self {
        Self {
            canvas: Canvas::new([512, 512]),
            menu_bar: MenuBar::new(),
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
        self.menu_bar.update(ctx);

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
