use egui::{Color32, ColorImage, Image, Sense, TextureHandle, TextureOptions, Ui};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Canvas {
    canvas: ColorImage,
    #[serde(skip)]
    texture: Option<TextureHandle>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            canvas: ColorImage::new([512, 512], Color32::GRAY),
            texture: None,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
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
    }

    fn paint_circle(&mut self, cx: usize, cy: usize) {
      let size = self.canvas.size;
      let radius = 10;

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