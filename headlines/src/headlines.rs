use eframe::egui::{self, Button, TopBottomPanel, Window};
use eframe::egui::{
    Align, Color32, FontData, FontDefinitions, FontFamily, Hyperlink, Label, Layout, RichText,
    Separator,
};
use std::iter::FromIterator;
use serde::{Serialize, Deserialize};
use confy;


pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const RED: Color32 = Color32::from_rgb(255, 0, 0);

#[derive(Serialize, Deserialize)]
pub struct HeadlinesConfig {
    pub dark_mode: bool,
    pub api_key: String
}

impl Default for HeadlinesConfig {
    fn default() -> Self {
        Self { dark_mode: Default::default(), api_key: String::new() }
    }
}

pub struct Headlines {
    articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig,
    pub api_key_initialized : bool
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

        let config : HeadlinesConfig = confy::load("headlines", None).unwrap_or_default();

        Headlines {
            articles: Vec::from_iter(iter),
            config,
            api_key_initialized: false
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

    pub fn render_config(&mut self, ctx: &egui::Context){
        Window::new("Configuration").show(ctx, |ui| {
            ui.label("Enter your API KEY for newsapi.org");
            let text_input = ui.text_edit_singleline(&mut self.config.api_key);
            if text_input.lost_focus() && ui.input().key_pressed(egui::Key::Enter){
                if let Err(e) = confy::store("headlines","headlines",  HeadlinesConfig {
                    dark_mode: self.config.dark_mode,
                    api_key: self.config.api_key.to_string()
                }){
                    tracing::error!("Failed saving app store: {}", e);
                }

                self.api_key_initialized = true;

                tracing::error!("api key set");
            }
            tracing::error!("{}", &self.config.api_key);
            ui.label("If you havn't registered forr the API_KEY, head over to");
            ui.hyperlink("https://newsapi.org");
        });
    }

}
