use eframe::egui;

use crate::config::Config;
use crate::storage::db::Database;
use crate::storage::models::TestResult;
use crate::ui::test_screen::TestState;
use crate::ui::theme;

pub struct TypingApp {
    config: Config,
    test_state: TestState,
    active_tab: Tab,
    json_data: String,
}

#[derive(PartialEq)]
enum Tab {
    Test,
    Gecmis,
}

impl TypingApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        theme::apply_dark_theme(&cc.egui_ctx);

        let config = Config::load();

        let json_data =
            include_str!("../assets/word_lists/genel_tr.json")
.to_string();

        Self {
            config,
            test_state: TestState::default(),
            active_tab: Tab::Test,
            json_data,
        }
    }
}

impl eframe::App for TypingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(theme::heading_text("fastshi"));
                ui.add_space(20.0);

                let test_btn = egui::Button::new(theme::button_text("test"))
                    .selected(self.active_tab == Tab::Test);
                if ui.add(test_btn).clicked() {
                    self.active_tab = Tab::Test;
                }

                let gecmis_btn = egui::Button::new(theme::button_text("geçmiş"))
                    .selected(self.active_tab == Tab::Gecmis);
                if ui.add(gecmis_btn).clicked() {
                    self.active_tab = Tab::Gecmis;
                }
            });
            ui.add_space(8.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab {
                Tab::Test => {
                    crate::ui::test_screen::show(ui, &mut self.test_state, &self.json_data);

                    if self.test_state.just_finished {
                        if let Some(ref metrics) = self.test_state.last_metrics {
                            let db = Database::open();
                            let result = TestResult {
                                id: None,
                                tarih: chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string(),
                                modu: self.test_state.test_modu_str.clone(),
                                sure_sn: self.test_state.sure_secenek,
                                hedef_kelime_sayisi: self.test_state.kelime_hedef,
                                wpm: metrics.wpm,
                                cpm: metrics.cpm,
                                dogruluk_yuzde: metrics.dogruluk_yuzde,
                                dogru_tus: metrics.dogru_tus,
                                yanlis_tus: metrics.yanlis_tus,
                                zorluk: self.test_state.zorluk.as_str().to_string(),
                            };
                            db.kaydet(&result);
                        }
                    }
                }
                Tab::Gecmis => {
                    crate::ui::stats_screen::show(ui);
                }
            }
        });
    }
}

fn configure_fonts(ctx: &egui::Context) {
    use egui::FontData;

    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "orbitron".to_owned(),
        std::sync::Arc::new(
            FontData::from_owned(include_bytes!("../assets/fonts/Orbitron.ttf").to_vec()),
        ),
    );
    fonts.font_data.insert(
        "spacemono".to_owned(),
        std::sync::Arc::new(
            FontData::from_owned(include_bytes!("../assets/fonts/SpaceMono-Regular.ttf").to_vec()),
        ),
    );
    fonts.font_data.insert(
        "spacemono_bold".to_owned(),
        std::sync::Arc::new(
            FontData::from_owned(include_bytes!("../assets/fonts/SpaceMono-Bold.ttf").to_vec()),
        ),
    );

    fonts
        .families
        .entry(egui::FontFamily::Name("orbitron".into()))
        .or_default()
        .insert(0, "orbitron".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Name("spacemono".into()))
        .or_default()
        .insert(0, "spacemono".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Name("spacemono_bold".into()))
        .or_default()
        .insert(0, "spacemono_bold".to_owned());

    ctx.set_fonts(fonts);
}
