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
    pub input_buf: String,
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
            input_buf: String::new(),
        }
    }
}

pub fn show(ui: &mut egui::Ui, state: &mut TestState, json_data: &str) {
    if state.just_finished {
        if let Some(ref metrics) = state.last_metrics {
            super::results_screen::show(ui, metrics, &state.test_modu_str, state.sure_secenek);
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui
                    .add(egui::Button::new(theme::button_text("yeniden başla")))
                    .clicked()
                {
                    start_test(state, json_data);
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
            });
            return;
        }
    }

    if !state.started {
        show_setup(ui, state, json_data);
    } else {
        show_typing(ui, state, json_data);
    }
}

fn start_test(state: &mut TestState, json_data: &str) {
    let hand = match state.mode {
        TestMode::RightHand => Some(Hand::Right),
        TestMode::LeftHand => Some(Hand::Left),
        _ => None,
    };

    let mut ws = WordSource::from_json(json_data);
    ws.filter(hand, state.zorluk.as_str());

    let total = state.kelime_hedef;
    let sure = if state.kelime_hedef.is_some() {
        0
    } else {
        state.sure_secenek
    };

    let mut engine = TypingEngine::new(total, sure);
    let initial_words: Vec<String> = (0..50).map(|_| ws.next_word()).collect();
    engine.load_words(initial_words);

    state.engine = Some(engine);
    state.word_source = Some(ws);
    state.started = true;
    state.just_finished = false;
    state.last_metrics = None;
    state.input_buf.clear();

    state.test_modu_str = match state.mode {
        TestMode::TwoHand => "iki_el",
        TestMode::RightHand => "sag_el",
        TestMode::LeftHand => "sol_el",
    }
    .to_string();
}

fn show_setup(ui: &mut egui::Ui, state: &mut TestState, json_data: &str) {
    ui.heading(theme::heading_text("fastshi"));
    ui.add_space(20.0);

    ui.label(theme::sub_heading_text("test modu"));
    ui.horizontal(|ui| {
        for (mode, label) in [
            (TestMode::TwoHand, "iki el"),
            (TestMode::RightHand, "sag el"),
            (TestMode::LeftHand, "sol el"),
        ] {
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
        for (z, label) in [
            (Zorluk::Kolay, "kolay"),
            (Zorluk::Orta, "orta"),
            (Zorluk::Zor, "zor"),
        ] {
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
    ui.label(theme::sub_heading_text("sure"));
    ui.horizontal(|ui| {
        for sn in [30, 60, 120] {
            let selected = state.sure_secenek == sn && state.kelime_hedef.is_none();
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
    ui.label(theme::sub_heading_text("kelime sayisi"));
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
    ws.filter(hand, state.zorluk.as_str());
    let available = ws.available_count();
    ui.label(theme::body_text(&format!("mevcut kelime: {}", available)));

    ui.add_space(10.0);
    let start_label = if let Some(k) = state.kelime_hedef {
        format!("{} kelime basla", k)
    } else {
        format!("{} sn basla", state.sure_secenek)
    };

    if ui
        .add(
            egui::Button::new(theme::button_text(&start_label))
                .min_size(egui::vec2(200.0, 40.0)),
        )
        .clicked()
    {
        start_test(state, json_data);
    }
}

fn show_typing(ui: &mut egui::Ui, state: &mut TestState, json_data: &str) {
    let engine = state.engine.as_mut().unwrap();
    let word_source = state.word_source.as_mut().unwrap();

    if engine.needs_more_words() {
        let new_words: Vec<String> = (0..30).map(|_| word_source.next_word()).collect();
        engine.words.extend(new_words);
        for w in engine.words[engine.typed_words.len()..].iter() {
            engine.typed_words.push(crate::typing::engine::TypedWord {
                word: w.clone(),
                typed_chars: vec![None; w.len()],
                completed: false,
            });
        }
    }

    // ust panel: sure / kelime sayaci
    ui.horizontal(|ui| {
        let elapsed = engine.elapsed_secs();
        if state.sure_secenek > 0 {
            let remaining = state.sure_secenek as f64 - elapsed;
            ui.label(theme::body_text(&format!("{:.0} sn", remaining.max(0.0))));
        } else {
            ui.label(theme::body_text(&format!(
                "{}/{} kelime",
                engine.words_completed,
                state.kelime_hedef.unwrap_or(0)
            )));
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let toplam = engine.dogru_tus + engine.yanlis_tus;
            let dogruluk = if toplam > 0 {
                (engine.dogru_tus as f64 / toplam as f64 * 100.0) as u32
            } else {
                100
            };
            ui.label(theme::body_text(&format!("%{}", dogruluk)));
        });
    });

    ui.add_space(8.0);

    // Paragraf alani — siyah kutu icinde
    let frame = egui::Frame::NONE
        .fill(theme::DARK_GRAY)
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin::same(16))
        .stroke(egui::Stroke::new(1.0_f32, theme::GRAY));

    frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        let layout = egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true);
        ui.with_layout(layout, |ui| {
            let word_index = engine.word_index;

            for i in word_index..engine.typed_words.len().min(word_index + 30) {
                let tw = &engine.typed_words[i];

                if i > word_index {
                    ui.label(
                        egui::RichText::new(" ")
                            .family(egui::FontFamily::Name("spacemono".into()))
                            .size(22.0)
                            .color(theme::GRAY),
                    );
                }

                for (ci, ch) in tw.word.chars().enumerate() {
                    let color = if tw.completed {
                        theme::WHITE
                    } else if i == word_index && ci < engine.cursor_in_word {
                        theme::WHITE
                    } else if i == word_index && ci == engine.cursor_in_word && engine.has_error {
                        egui::Color32::from_rgb(255, 80, 80)
                    } else if i == word_index && ci == engine.cursor_in_word {
                        egui::Color32::from_rgb(255, 255, 0)
                    } else if i == word_index {
                        theme::GRAY
                    } else {
                        egui::Color32::from_rgb(80, 80, 80)
                    };

                    let rt = egui::RichText::new(ch)
                        .family(egui::FontFamily::Name("spacemono".into()))
                        .size(22.0)
                        .color(color);
                    ui.label(rt);
                }
            }
        });
    });

    ui.add_space(12.0);

    // Input kutusu
    let response = ui.add(
        egui::TextEdit::singleline(&mut state.input_buf)
            .font(egui::TextStyle::Monospace)
            .desired_width(ui.available_width())
            .hint_text(theme::small_text("yazmaya baslayin...")),
    );

    response.request_focus();

    // Enter veya bosluk ile kelimeyi ilerlet
    let input = state.input_buf.clone();
    if !input.is_empty() {
        if input.ends_with(' ') {
            let yazilan = input.trim_end().to_string();
            for ch in yazilan.chars() {
                engine.on_key(ch);
            }
            engine.on_space();
            state.input_buf.clear();
        } else {
            state.input_buf.clear();
            for ch in input.chars() {
                engine.on_key(ch);
            }
        }
    }

    if engine.is_finished() {
        let metrics =
            Metrics::hesapla(engine.dogru_tus, engine.yanlis_tus, engine.elapsed_secs());
        state.last_metrics = Some(metrics);
        state.just_finished = true;
        state.started = false;
    }

    ui.ctx().request_repaint();
}
