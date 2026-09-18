use eframe::egui;

use crate::hand_mode::key_map::Hand;
use crate::typing::engine::{TestMode, TypingEngine, Zorluk};
use crate::typing::metrics::Metrics;
use crate::typing::word_source::WordSource;
use crate::ui::keyboard_widget::{self, KeyboardWidget};
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
    pub keyboard: KeyboardWidget,
    pub text_input_id: Option<egui::Id>,
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
            keyboard: KeyboardWidget::default(),
            text_input_id: None,
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
                    .add(egui::Button::new(theme::button_text("yeniden basla")))
                    .clicked()
                {
                    start_test(state, json_data);
                }
                if ui
                    .add(egui::Button::new(theme::button_text("menuye don")))
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
        show_typing(ui, state);
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
    state.keyboard = KeyboardWidget::default();

    state.test_modu_str = match state.mode {
        TestMode::TwoHand => "iki_el",
        TestMode::RightHand => "sag_el",
        TestMode::LeftHand => "sol_el",
    }
    .to_string();
}

fn show_setup(ui: &mut egui::Ui, state: &mut TestState, json_data: &str) {
    ui.heading(theme::heading_text("fastshi"));
    ui.add_space(16.0);

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
                .min_size(egui::vec2(90.0, 28.0));
            if ui.add(btn).clicked() {
                state.mode = mode;
            }
        }
    });

    ui.add_space(8.0);
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
                .min_size(egui::vec2(70.0, 28.0));
            if ui.add(btn).clicked() {
                state.zorluk = z;
            }
        }
    });

    ui.add_space(8.0);
    ui.label(theme::sub_heading_text("sure"));
    ui.horizontal(|ui| {
        for sn in [30, 60, 120] {
            let selected = state.sure_secenek == sn && state.kelime_hedef.is_none();
            let btn = egui::Button::new(theme::button_text(&format!("{} sn", sn)))
                .selected(selected)
                .min_size(egui::vec2(70.0, 28.0));
            if ui.add(btn).clicked() {
                state.sure_secenek = sn;
                state.kelime_hedef = None;
            }
        }
    });

    ui.add_space(8.0);
    ui.label(theme::sub_heading_text("kelime sayisi"));
    ui.horizontal(|ui| {
        for k in [25, 50, 100] {
            let selected = state.kelime_hedef == Some(k);
            let btn = egui::Button::new(theme::button_text(&format!("{}", k)))
                .selected(selected)
                .min_size(egui::vec2(55.0, 28.0));
            if ui.add(btn).clicked() {
                state.kelime_hedef = Some(k);
                state.sure_secenek = 0;
            }
        }
    });

    ui.add_space(16.0);

    let start_label = if let Some(k) = state.kelime_hedef {
        format!("{} kelime basla", k)
    } else {
        format!("{} sn basla", state.sure_secenek)
    };

    if ui
        .add(
            egui::Button::new(theme::button_text(&start_label))
                .min_size(egui::vec2(180.0, 36.0)),
        )
        .clicked()
    {
        start_test(state, json_data);
    }

    ui.add_space(16.0);

    let frame = egui::Frame::NONE
        .fill(theme::DARK_GRAY)
        .corner_radius(egui::CornerRadius::same(6))
        .inner_margin(egui::Margin::same(10))
        .stroke(egui::Stroke::new(1.0_f32, theme::GRAY));

    frame.show(ui, |ui| {
        ui.label(theme::small_text("klavye duzeni — turkce q"));
        state.keyboard.show(ui);
        ui.add_space(4.0);
        keyboard_widget::show_legend(ui);
    });
}

fn show_typing(ui: &mut egui::Ui, state: &mut TestState) {
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

    // ust satir: sure + dogruluk
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

    ui.add_space(4.0);

    // Paragraf alani — kucuk kutu, kelimeler sarilmali
    let frame = egui::Frame::NONE
        .fill(theme::DARK_GRAY)
        .corner_radius(egui::CornerRadius::same(6))
        .inner_margin(egui::Margin::same(10))
        .stroke(egui::Stroke::new(1.0_f32, theme::GRAY));

    frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        let available_width = ui.available_width();
        let mut current_line_width = 0.0_f32;
        let char_width = 9.6; // spacemono 16pt icin tahmini genislik
        let space_width = 5.0;
        let word_index = engine.word_index;

        ui.vertical(|ui| {
            ui.horizontal_wrapped(|ui| {
                for i in word_index..engine.typed_words.len().min(word_index + 25) {
                    let tw = &engine.typed_words[i];
                    let word_pixel_width = tw.word.len() as f32 * char_width;

                    // Eger bu kelime sigmiyorsa yeni satira gec
                    if i > word_index && current_line_width + space_width + word_pixel_width > available_width {
                        ui.end_row();
                        current_line_width = 0.0;
                    }

                    if i > word_index {
                        ui.label(
                            egui::RichText::new(" ")
                                .family(egui::FontFamily::Name("spacemono".into()))
                                .size(16.0)
                                .color(theme::GRAY),
                        );
                        current_line_width += space_width;
                    }

                    for (ci, ch) in tw.word.chars().enumerate() {
                        let color = if tw.completed {
                            theme::WHITE
                        } else if i == word_index && ci < engine.cursor_in_word {
                            theme::WHITE
                        } else if i == word_index && ci == engine.cursor_in_word && engine.has_error {
                            egui::Color32::from_rgb(255, 80, 80)
                        } else if i == word_index && ci == engine.cursor_in_word {
                            let finger = keyboard_widget::finger_of(ch);
                            finger.color()
                        } else if i == word_index {
                            theme::GRAY
                        } else {
                            egui::Color32::from_rgb(80, 80, 80)
                        };

                        let rt = egui::RichText::new(ch)
                            .family(egui::FontFamily::Name("spacemono".into()))
                            .size(16.0)
                            .color(color);
                        ui.label(rt);
                        current_line_width += char_width;
                    }
                }
            });
        });
    });

    ui.add_space(6.0);

    // Input kutusu
    let response = ui.add(
        egui::TextEdit::singleline(&mut state.input_buf)
            .font(egui::TextStyle::Monospace)
            .desired_width(ui.available_width())
            .hint_text(theme::small_text("yazmaya baslayin...")),
    );
    response.request_focus();

    // Input isleme — eski buffer'daki tum karakterleri isle
    let input = state.input_buf.clone();
    if !input.is_empty() {
        state.input_buf.clear();

        for ch in input.chars() {
            if ch == ' ' {
                // space ile kelimeyi bitir
                engine.on_space();
                state.keyboard.add_press("SPACE".to_string());
                // bir sonraki kelimenin ilk harfini goster
                if let Some(next_ch) = engine.current_word().chars().nth(engine.cursor_in_word) {
                    state.keyboard.set_highlight(Some(next_ch));
                } else {
                    state.keyboard.set_highlight(None);
                }
            } else {
                engine.on_key(ch);
                state.keyboard.add_press(ch.to_string().to_uppercase());
                if let Some(next_ch) = engine.current_word().chars().nth(engine.cursor_in_word) {
                    state.keyboard.set_highlight(Some(next_ch));
                } else {
                    state.keyboard.set_highlight(None);
                }
            }
        }
    }

    ui.add_space(6.0);

    // Klavye widget'i + legend
    let frame = egui::Frame::NONE
        .fill(theme::DARK_GRAY)
        .corner_radius(egui::CornerRadius::same(6))
        .inner_margin(egui::Margin::same(8))
        .stroke(egui::Stroke::new(1.0_f32, theme::GRAY));

    frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                state.keyboard.show(ui);
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                keyboard_widget::show_legend(ui);
            });
        });
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
