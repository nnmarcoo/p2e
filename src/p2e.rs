use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context};

pub struct P2e {
  counter: u32,
}

impl P2e {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
          counter: 0,
        }
    }
}

impl App for P2e {
    fn update(&mut self, _ctx: &Context, _frame: &mut Frame) {
      CentralPanel::default().show(_ctx, |ui| {
        ui.label(format!("Counter: {}", self.counter));
        if ui.button("Increment").clicked() {
          self.counter += 1;
        }
      });
    }
}
