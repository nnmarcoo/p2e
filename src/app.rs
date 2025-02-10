use eframe::App;
use egui::{Color32, ColorImage, Image, Sense, TextureHandle, TextureOptions, Window};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    canvas: ColorImage,
    #[serde(skip)]
    texture: Option<TextureHandle>,
    brush_size: usize,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            canvas: ColorImage::new([512, 512], Color32::GRAY),
            texture: None,
            brush_size: 10,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl App for TemplateApp {
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
                    let size = self.canvas.size;

                    let _texture = self.texture.get_or_insert_with(|| {
                        ctx.load_texture(
                            "canvas_texture",
                            self.canvas.clone(),
                            TextureOptions::default(),
                        )
                    });

                    if let Some(texture) = &self.texture {
                        let response = ui.add(Image::new(texture).sense(Sense::drag()));
                        if let Some(pos) = response.interact_pointer_pos() {
                            let uv = pos - response.rect.min;
                            let px = (uv.x * size[0] as f32 / response.rect.width()) as usize;
                            let py = (uv.y * size[1] as f32 / response.rect.height()) as usize;
                            if px < size[0] && py < size[1] {
                                self.paint_circle(px, py);
                                self.texture = Some(ctx.load_texture(
                                    "canvas_texture",
                                    self.canvas.clone(),
                                    TextureOptions::default(),
                                ));
                            }
                        }
                    }
                });
        });
    }
}

impl TemplateApp {
    fn paint_circle(&mut self, cx: usize, cy: usize) {
        let size = self.canvas.size;
        let radius = self.brush_size as isize;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= radius * radius {
                    let x = cx as isize + dx;
                    let y = cy as isize + dy;

                    if x >= 0 && y >= 0 && x < size[0] as isize && y < size[1] as isize {
                        self.canvas.pixels[y as usize * size[0] + x as usize] = Color32::BLACK;
                    }
                }
            }
        }
    }
}
