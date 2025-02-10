use eframe::{App, CreationContext, Frame};
use egui::Context;



pub struct P2e {

}

impl P2e {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
        }
    }
}

impl App for P2e {
  fn update(&mut self, _ctx: &Context, _frame: &mut Frame) {
  }
}