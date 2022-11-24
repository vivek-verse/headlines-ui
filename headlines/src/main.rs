mod headlines;
use crate::headlines::Headlines;
use eframe::egui;
use eframe::{egui::{CentralPanel, ScrollArea, Vec2}, App};

impl App for Headlines {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().show(ui, |ui|{
                self.render_news_cards(ui);
            });
       });
        self.configure_fonts(ctx);
   }
}

fn main() {
    let app = Headlines::new();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540.0, 960.0));
    eframe::run_native("Headlines", native_options, Box::new(|_| Box::new(app)));
}
