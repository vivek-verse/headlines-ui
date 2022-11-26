use eframe::egui::{self, Button, TopBottomPanel};
use eframe::egui::{
    Align, Color32, FontData, FontDefinitions, FontFamily, Hyperlink, Label, Layout, RichText,
    Separator,
};
use std::iter::FromIterator;

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const RED: Color32 = Color32::from_rgb(255, 0, 0);
pub struct HeadlinesConfig {
    pub dark_mode: bool,
}

impl HeadlinesConfig {
    fn new() -> Self {
        Self { dark_mode: true }
    }
}

pub struct Headlines {
    articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig,
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
            config: HeadlinesConfig::new(),
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

            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }

            ui.add_space(PADDING);
            let desc =
                Label::new(RichText::new(&a.desc).text_style(eframe::egui::TextStyle::Button));
            ui.add(desc);

            if self.config.dark_mode {
                ui.style_mut().visuals.hyperlink_color = CYAN;
            } else {
                ui.style_mut().visuals.hyperlink_color = RED;
            }

            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more ⤴", &a.url));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub fn render_top_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
                    if close_btn.clicked() {
                        frame.close();
                    }
                    let refresh_btn = ui.add(Button::new("🔄"));
                    let theme_btn = ui.add(Button::new({
                        if self.config.dark_mode {
                            "🌞"
                        } else {
                            "🌙"
                        }
                    }));
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                    }
                });

                ui.add_space(10.);
            });

            ui.add_space(10.);
        });
    }
}
