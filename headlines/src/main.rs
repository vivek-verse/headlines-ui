mod headlines;
use crate::headlines::{Headlines, PADDING};
use eframe::egui::{self, Hyperlink, Label, RichText, TopBottomPanel, Ui, Visuals};
use eframe::{
    egui::{CentralPanel, ScrollArea, Separator, Vec2},
    App,
};
use tracing_subscriber;
impl App for Headlines {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }


        if !self.api_key_initialized {
            self.render_config(ctx);            
        }else{
            self.render_top_panel(ctx, frame);
            CentralPanel::default().show(ctx, |ui| {
                render_header(ui);
                ScrollArea::both().show(ui, |ui| {
                    self.render_news_cards(ui);
                });
                render_footer(ctx);
            });
            self.configure_fonts(ctx);
        }

    }
}

fn render_footer(ctx: &egui::Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);

            ui.add(Label::new(
                RichText::new("API source: newsapi.org").monospace(),
            ));

            ui.add(Hyperlink::from_label_and_url(
                RichText::new("Made with egui").text_style(eframe::egui::TextStyle::Monospace),
                "https://github.com/emilk/egui",
            ));

            ui.add(Hyperlink::from_label_and_url(
                RichText::new("vivek-verse/headlines-app")
                    .text_style(eframe::egui::TextStyle::Monospace),
                "https://github.com/vivek-verse/headlines-app",
            ));

            ui.add_space(10.);
        });
    });
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn main() {
    tracing_subscriber::fmt::init();
    let app = Headlines::new();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540.0, 960.0));
    eframe::run_native("Headlines", native_options, Box::new(|_| Box::new(app)));
}
