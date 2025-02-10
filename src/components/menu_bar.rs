use egui::{menu, widgets, TopBottomPanel, ViewportCommand};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MenuBar {}

impl MenuBar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, ctx: &egui::Context) {
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
    }
}
