use egui::{Color32, ColorImage, Image, Sense, TextureHandle, TextureOptions, Ui};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Canvas {
    canvas: ColorImage,
    size: [usize; 2],
    brush_thickness: usize,
    brush_color: [u8; 3],
    #[serde(skip)]
    texture: Option<TextureHandle>,
    #[serde(skip)]
    last_pos: Option<(usize, usize)>,
}

impl Canvas {
    pub fn new(size: [usize; 2]) -> Self {
        Self {
            size,
            canvas: ColorImage::new(size, Color32::WHITE),
            brush_thickness: 3,
            brush_color: [0, 0, 0],
            texture: None,
            last_pos: None,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let size = self.canvas.size;

        ui.horizontal(|ui| {
            ui.label("Brush Thickness:");
            ui.add(egui::Slider::new(&mut self.brush_thickness, 1..=10));
            ui.label("Brush Color:");
            ui.color_edit_button_srgb(&mut self.brush_color);
            if ui.button("Clear").clicked() {
                self.clear_canvas();
            }
        });

        self.texture.get_or_insert_with(|| {
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
                    if let Some((lx, ly)) = self.last_pos {
                        self.paint_line(lx, ly, px, py);
                    } else {
                        self.paint_circle(px, py);
                    }

                    self.last_pos = Some((px, py));

                    self.texture = Some(ctx.load_texture(
                        "canvas_texture",
                        self.canvas.clone(),
                        TextureOptions::default(),
                    ));
                }
            } else {
                self.last_pos = None;
            }
        }
    }

    fn clear_canvas(&mut self) {
        self.canvas = ColorImage::new(self.size, Color32::WHITE);
        self.texture = None;
    }

    fn paint_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let dx = (x1 as isize - x0 as isize).abs();
        let dy = (y1 as isize - y0 as isize).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = x0 as isize;
        let mut y = y0 as isize;

        while x != x1 as isize || y != y1 as isize {
            if x >= 0
                && y >= 0
                && x < self.canvas.size[0] as isize
                && y < self.canvas.size[1] as isize
            {
                self.paint_circle(x as usize, y as usize);
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn paint_circle(&mut self, cx: usize, cy: usize) {
        let size = self.canvas.size;
        let radius = self.brush_thickness as isize;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= radius * radius {
                    let x = cx as isize + dx;
                    let y = cy as isize + dy;

                    if x >= 0 && y >= 0 && x < size[0] as isize && y < size[1] as isize {
                        self.canvas.pixels[y as usize * size[0] + x as usize] = Color32::from_rgb(
                            self.brush_color[0],
                            self.brush_color[1],
                            self.brush_color[2],
                        );
                    }
                }
            }
        }
    }
}
