use eframe::egui;

use crate::hand_mode::key_map::Hand;
use crate::typing::engine::{TestMode, TypingEngine, Zorluk};
use crate::typing::metrics::Metrics;
use crate::typing::word_source::WordSource;
use super::theme;

pub struct TestState {
    pub engine: Option<TypingEngine>,
    pub word_source: Option<WordSource>,
    pub mode: TestMode,
    pub zorluk: Zorluk,
    pub sure_secenek: u64,
    pub kelime_hedef: Option<u32>,
    pub started: bool,
    pub just_finished: bool,
    pub last_metrics: Option<Metrics>,
    pub test_modu_str: String,
}

impl Default for TestState {
    fn default() -> Self {
        Self {
            engine: None,
            word_source: None,
            mode: TestMode::TwoHand,
            zorluk: Zorluk::Orta,
            sure_secenek: 60,
            kelime_hedef: None,
            started: false,
            just_finished: false,
            last_metrics: None,
            test_modu_str: "iki_el".to_string(),
        }
    }
}

pub fn show(ui: &mut egui::Ui, state: &mut TestState, json_data: &str) {
    if state.just_finished {
        if let Some(ref metrics) = state.last_metrics {
            super::results_screen::show(ui, metrics, &state.test_modu_str, state.sure_secenek);
            if ui
                .add(egui::Button::new(theme::button_text("yeniden başla")))
                .clicked()
            {
                state.just_finished = false;
                state.last_metrics = None;
                state.started = false;
                state.engine = None;
            }
            if ui
                .add(egui::Button::new(theme::button_text("menüye dön")))
                .clicked()
            {
                state.just_finished = false;
                state.last_metrics = None;
                state.started = false;
                state.engine = None;
            }
            return;
        }
    }

    if !state.started {
        show_setup(ui, state, json_data);
    } else {
        show_typing(ui, state);
    }
}

fn show_setup(ui: &mut egui::Ui, state: &mut TestState, json_data: &str) {
    ui.heading(theme::heading_text("yazım antrenörü"));
    ui.add_space(20.0);

    ui.label(theme::sub_heading_text("test modu"));
    ui.horizontal(|ui| {
        let modes = [
            (TestMode::TwoHand, "iki el"),
            (TestMode::RightHand, "sağ el"),
            (TestMode::LeftHand, "sol el"),
        ];
        for (mode, label) in modes {
            let selected = state.mode == mode;
            let btn = egui::Button::new(theme::button_text(label))
                .selected(selected)
                .min_size(egui::vec2(100.0, 30.0));
            if ui.add(btn).clicked() {
                state.mode = mode;
            }
        }
    });

    ui.add_space(10.0);
    ui.label(theme::sub_heading_text("zorluk"));
    ui.horizontal(|ui| {
        let zorluklar = [
            (Zorluk::Kolay, "kolay"),
            (Zorluk::Orta, "orta"),
            (Zorluk::Zor, "zor"),
        ];
        for (z, label) in zorluklar {
            let selected = state.zorluk == z;
            let btn = egui::Button::new(theme::button_text(label))
                .selected(selected)
                .min_size(egui::vec2(80.0, 30.0));
            if ui.add(btn).clicked() {
                state.zorluk = z;
            }
        }
    });

    ui.add_space(10.0);
    ui.label(theme::sub_heading_text("süre"));
    ui.horizontal(|ui| {
        for sn in [30, 60, 120] {
            let selected = state.sure_secenek == sn;
            let btn = egui::Button::new(theme::button_text(&format!("{} sn", sn)))
                .selected(selected)
                .min_size(egui::vec2(80.0, 30.0));
            if ui.add(btn).clicked() {
                state.sure_secenek = sn;
                state.kelime_hedef = None;
            }
        }
    });

    ui.add_space(10.0);
    ui.label(theme::sub_heading_text("kelime sayısı"));
    ui.horizontal(|ui| {
        for k in [25, 50, 100] {
            let selected = state.kelime_hedef == Some(k);
            let btn = egui::Button::new(theme::button_text(&format!("{}", k)))
                .selected(selected)
                .min_size(egui::vec2(60.0, 30.0));
            if ui.add(btn).clicked() {
                state.kelime_hedef = Some(k);
                state.sure_secenek = 0;
            }
        }
    });

    ui.add_space(20.0);

    let hand = match state.mode {
        TestMode::RightHand => Some(Hand::Right),
        TestMode::LeftHand => Some(Hand::Left),
        _ => None,
    };

    let mut ws = WordSource::from_json(json_data);
    let zorluk_str = state.zorluk.as_str();
    ws.filter(hand, zorluk_str);
    let available = ws.available_count();

    ui.label(theme::body_text(&format!("mevcut kelime: {}", available)));

    if available < 5 {
        ui.label(theme::small_text("yetersiz kelime — hece pratiğine geçilebilir"));
    }

    ui.add_space(10.0);
    let start_text = if let Some(k) = state.kelime_hedef {
        theme::button_text(&format!("{} kelime başla", k))
    } else {
        theme::button_text(&format!("{} sn başla", state.sure_secenek))
    };

    if ui
        .add(egui::Button::new(start_text).min_size(egui::vec2(200.0, 40.0)))
        .clicked()
    {
        let total = state.kelime_hedef;
        let sure = if state.kelime_hedef.is_some() {
            0
        } else {
            state.sure_secenek
        };

        let mut engine = TypingEngine::new(total, sure);
        let first_word = ws.next_word();
        engine.set_word(first_word);

        state.engine = Some(engine);
        state.word_source = Some(ws);
        state.started = true;

        state.test_modu_str = match state.mode {
            TestMode::TwoHand => "iki_el",
            TestMode::RightHand => "sag_el",
            TestMode::LeftHand => "sol_el",
        }
        .to_string();
    }
}

fn show_typing(ui: &mut egui::Ui, state: &mut TestState) {
    let engine = state.engine.as_mut().unwrap();
    let word_source = state.word_source.as_mut().unwrap();

    ui.heading(theme::sub_heading_text("yazıyor..."));

    let elapsed = engine.elapsed_secs();
    if state.sure_secenek > 0 {
        let remaining = state.sure_secenek as f64 - elapsed;
        ui.label(theme::body_text(&format!("{:.0} sn kaldı", remaining.max(0.0))));
    } else {
        ui.label(theme::body_text(&format!(
            "{:.0} kelime / {}",
            engine.words_completed,
            state.kelime_hedef.unwrap_or(0)
        )));
    }

    ui.add_space(10.0);

    let current_word = &engine.current_word;
    let cursor = engine.cursor;
    let has_error = engine.has_error;

    ui.horizontal(|ui| {
        for (i, ch) in current_word.chars().enumerate() {
            let color = if i < cursor {
                theme::WHITE
                } else if i == cursor && has_error {
                    egui::Color32::from_rgb(255, 80, 80)
                } else if i == cursor {
                    egui::Color32::from_rgb(255, 255, 0)
            } else {
                theme::GRAY
            };

            let text = egui::RichText::new(ch)
                .family(egui::FontFamily::Name("spacemono".into()))
                .size(28.0)
                .color(color);
            ui.label(text);
        }
    });

    ui.add_space(5.0);
    ui.label(theme::small_text(&format!("kelime: {}", engine.words_completed + 1)));

    ui.add_space(20.0);

    ui.input(|i| {
        for event in &i.events {
            if let egui::Event::Text(text) = event {
                for ch in text.chars() {
                    if ch == ' ' && engine.cursor >= engine.current_word.len() {
                        let next = word_source.next_word();
                        engine.set_word(next);
                    } else if ch != ' ' {
                        engine.on_key(ch);
                    }
                }
            }
            if let egui::Event::Key {
                key: _,
                physical_key: _,
                pressed: _,
                repeat: _,
                modifiers: _,
            } = event
            {
                // hata modunda backspace engellendi
            }
        }
    });

    if engine.is_finished() {
        let metrics =
            Metrics::hesapla(engine.dogru_tus, engine.yanlis_tus, engine.elapsed_secs());
        state.last_metrics = Some(metrics);
        state.just_finished = true;
        state.started = false;
    }

    ui.ctx().request_repaint();
}
