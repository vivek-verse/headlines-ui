use eframe::egui::{self, Button, TopBottomPanel};
use eframe::egui::{
    Align, Color32, FontData, FontDefinitions, FontFamily, Hyperlink, Label, Layout, RichText,
    Separator,
};
use std::iter::FromIterator;

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

#[derive(Default)]
pub struct Headlines {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://example.com/{}", a),
        });

        Headlines {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn configure_fonts(&mut self, ctx: &egui::Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_owned(),
            FontData::from_static(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
        );
        font_def
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());
        ctx.set_fonts(font_def);
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            // ui.add_space(PADDING);
            let title = format!("▶ {}", a.title);
            ui.colored_label(WHITE, title);
            ui.add_space(PADDING);
            let desc =
                Label::new(RichText::new(&a.desc).text_style(eframe::egui::TextStyle::Button));
            ui.add(desc);

            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more ⤴", &a.url));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub fn render_top_panel(&self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.add(Label::new(
                        RichText::new("📓").text_style(eframe::egui::TextStyle::Heading),
                    ));
                });

                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    let close_btn = ui.add(Button::new("❌"));
                    let refresh_btn = ui.add(Button::new("🔄"));
                    let theme_btn = ui.add(Button::new("🌙"));
                });

                ui.add_space(10.);
            });

            ui.add_space(10.);
        });
    }
}
