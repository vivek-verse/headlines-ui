mod headlines;
use std::{sync::mpsc::channel, thread};

use crate::headlines::{Headlines, PADDING, NewsCardData};
use eframe::egui::{self, Hyperlink, Label, RichText, TopBottomPanel, Ui, Visuals};
use eframe::{
    egui::{CentralPanel, ScrollArea, Separator, Vec2},
    App,
};
use newslib::NewsAPI;
use tracing_subscriber;

impl App for Headlines {

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {
        if !self.data_is_set && !self.config.api_key.is_empty(){

            let api_key = &self.config.api_key;
            
            let api_key = api_key.to_string();

            let (news_tx, news_rx) = channel();

            self.news_rx = Some(news_rx);
        
            let response = NewsAPI::new(&api_key).fetch().unwrap();
            
            thread::spawn(move ||{
                    let resp_articles = response.articles();
                    for a in resp_articles.iter(){
                        let news = NewsCardData {
                            title : a.title().to_string(),
                            url: a.url().to_string(),
                            desc: a.desc().map(|s| s.to_string()).unwrap_or("...".to_string())
                        };
        
                        if let Err(e) = news_tx.send(news){
                            tracing::error!("Error sending news data: {}", e);
                        }
                    }
            });
            self.data_is_set = true;
        }
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if self.news_rx.is_some() {
            self.preload_articles();
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
