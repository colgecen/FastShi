use eframe::egui;

use crate::config::Config;
use crate::storage::db::Database;
use crate::storage::models::TestResult;
use crate::ui::test_screen::TestState;
use crate::ui::theme;

pub struct TypingApp {
    config: Config,
    test_state: TestState,
    json_data: String,
    finger_image: Option<egui::TextureHandle>,
    logo_image: Option<egui::TextureHandle>,
    sound: Option<crate::sound::SoundManager>,
}

impl TypingApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        theme::apply_dark_theme(&cc.egui_ctx);

        let config = Config::load();

        let json_data =
            include_str!("../assets/word_lists/genel_tr.json").to_string();

        let finger_image = None;

        Self {
            config,
            test_state: TestState::default(),
            json_data,
            finger_image,
            logo_image: None,
            sound: crate::sound::SoundManager::new(),
        }
    }

    fn ensure_finger_image(&mut self, ctx: &egui::Context) {
        if self.finger_image.is_some() {
            return;
        }
        let bytes = include_bytes!("../assets/images/ten-finger.png");
        let image = image::load_from_memory(bytes).ok();
        if let Some(img) = image {
            let size = [img.width() as usize, img.height() as usize];
            let rgba = img.to_rgba8();
            let pixels = rgba.into_raw();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
            let handle = ctx.load_texture("finger_map", color_image, egui::TextureOptions::default());
            self.finger_image = Some(handle);
        }
    }

    fn ensure_logo_image(&mut self, ctx: &egui::Context) {
        if self.logo_image.is_some() {
            return;
        }
        let bytes = include_bytes!("../assets/images/FastShi.png");
        let image = image::load_from_memory(bytes).ok();
        if let Some(img) = image {
            let size = [img.width() as usize, img.height() as usize];
            let rgba = img.to_rgba8();
            let pixels = rgba.into_raw();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
            let handle = ctx.load_texture("logo", color_image, egui::TextureOptions::default());
            self.logo_image = Some(handle);
        }
    }
}

impl eframe::App for TypingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ensure_finger_image(ctx);
        self.ensure_logo_image(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::test_screen::show(
                ui,
                &mut self.test_state,
                &self.json_data,
                self.finger_image.as_ref(),
                self.logo_image.as_ref(),
                self.sound.as_ref(),
            );

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
        });
    }
}

fn configure_fonts(ctx: &egui::Context) {
    use egui::FontData;

    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "rajdhani".to_owned(),
        std::sync::Arc::new(
            FontData::from_owned(include_bytes!("../assets/fonts/Rajdhani-SemiBold.ttf").to_vec()),
        ),
    );
    fonts.font_data.insert(
        "rajdhani_bold".to_owned(),
        std::sync::Arc::new(
            FontData::from_owned(include_bytes!("../assets/fonts/Rajdhani-Bold.ttf").to_vec()),
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
        .entry(egui::FontFamily::Name("rajdhani".into()))
        .or_default()
        .insert(0, "rajdhani".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Name("rajdhani_bold".into()))
        .or_default()
        .insert(0, "rajdhani_bold".to_owned());

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
