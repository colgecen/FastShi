use eframe::egui;

use crate::hand_mode::key_map::Hand;
use crate::storage::db::Database;
use crate::typing::engine::{TestMode, TypingEngine, Zorluk};
use crate::typing::metrics::Metrics;
use crate::typing::word_source::WordSource;
use crate::ui::keyboard_widget;
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
    pub show_history: bool,
    pub show_next_key: bool,
    pub pressed_key: Option<char>,
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
            show_history: false,
            show_next_key: true,
            pressed_key: None,
        }
    }
}

pub fn show(ui: &mut egui::Ui, state: &mut TestState, json_data: &str, _finger_image: Option<&egui::TextureHandle>, _logo_image: Option<&egui::TextureHandle>, sound: Option<&crate::sound::SoundManager>) {
    // Her zaman kelimeleri yuklu tut — eger engine yoksa baslat
    if state.engine.is_none() {
        init_engine(state, json_data);
    }

    // Gecmis popup
    if state.show_history {
        show_history_window(ui.ctx(), &mut state.show_history);
    }

    ui.add_space(6.0);

    // === AYARLAR (kompakt tek satir) ===
    ui.horizontal_wrapped(|ui| {
        ui.label(theme::sub_heading_text("mod"));
        for (mode, label) in [
            (TestMode::TwoHand, "iki el"),
                          (TestMode::RightHand, "sag el"),
                          (TestMode::LeftHand, "sol el"),
        ] {
            let selected = state.mode == mode;
            let btn = egui::Button::new(theme::button_text(label))
            .selected(selected)
            .min_size(egui::vec2(60.0, 24.0));
            if ui.add(btn).clicked() {
                state.mode = mode;
                if state.started || state.just_finished {
                    reload_engine(state, json_data);
                }
            }
        }

        ui.add_space(8.0);
        ui.label(theme::sub_heading_text("zorluk"));
        for (z, label) in [
            (Zorluk::Kolay, "kolay"),
                          (Zorluk::Orta, "orta"),
                          (Zorluk::Zor, "zor"),
        ] {
            let selected = state.zorluk == z;
            let btn = egui::Button::new(theme::button_text(label))
            .selected(selected)
            .min_size(egui::vec2(50.0, 24.0));
            if ui.add(btn).clicked() {
                state.zorluk = z;
                if state.started || state.just_finished {
                    reload_engine(state, json_data);
                }
            }
        }

        ui.add_space(8.0);
        ui.label(theme::sub_heading_text("sure"));
        for sn in [30, 60, 120] {
            let selected = state.sure_secenek == sn && state.kelime_hedef.is_none();
            let btn = egui::Button::new(theme::button_text(&format!("{}sn", sn)))
            .selected(selected)
            .min_size(egui::vec2(45.0, 24.0));
            if ui.add(btn).clicked() {
                state.sure_secenek = sn;
                state.kelime_hedef = None;
                if state.started || state.just_finished {
                    reload_engine(state, json_data);
                }
            }
        }

        ui.add_space(8.0);
        ui.label(theme::sub_heading_text("kelime"));
        for k in [25, 50, 100] {
            let selected = state.kelime_hedef == Some(k);
            let btn = egui::Button::new(theme::button_text(&format!("{}", k)))
            .selected(selected)
            .min_size(egui::vec2(40.0, 24.0));
            if ui.add(btn).clicked() {
                state.kelime_hedef = Some(k);
                state.sure_secenek = 0;
                if state.started || state.just_finished {
                    reload_engine(state, json_data);
                }
            }
        }

        ui.add_space(16.0);

        let harf_label = if state.show_next_key { "harf: goster" } else { "harf: gizle" };
        let harf_btn = egui::Button::new(theme::button_text(harf_label))
            .selected(state.show_next_key)
            .min_size(egui::vec2(80.0, 24.0));
        if ui.add(harf_btn).clicked() {
            state.show_next_key = !state.show_next_key;
        }

        ui.add_space(8.0);
        let gecmis_btn = egui::Button::new(theme::button_text("gecmis"));
        if ui.add(gecmis_btn).clicked() {
            state.show_history = true;
        }
    });

    ui.add_space(4.0);

    // === DURUM BILGISI ===
    if state.started {
        ui.horizontal(|ui| {
            let engine = state.engine.as_ref().unwrap();
            let elapsed = engine.elapsed_secs();
            if state.sure_secenek > 0 {
                let remaining = state.sure_secenek as f64 - elapsed;
                ui.label(theme::body_text(&format!("{:.0} sn kaldi", remaining.max(0.0))));
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
    } else if state.just_finished {
        if let Some(ref metrics) = state.last_metrics {
            let mut open = true;
            egui::Window::new("sonuclar")
                .open(&mut open)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .fixed_size([320.0, 200.0])
                .show(ui.ctx(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        ui.label(theme::sub_heading_text(&format!("mod: {}", state.test_modu_str)));
                        ui.label(theme::body_text(&format!("sure: {}sn", state.sure_secenek)));
                        ui.add_space(8.0);
                        ui.label(theme::body_text(&format!("wpm: {:.1}", metrics.wpm)));
                        ui.label(theme::body_text(&format!("cpm: {:.1}", metrics.cpm)));

                        let dogruluk_rengi = if metrics.dogruluk_yuzde >= 90.0 {
                            theme::WHITE
                        } else if metrics.dogruluk_yuzde >= 70.0 {
                            theme::LIGHT_GRAY
                        } else {
                            egui::Color32::from_rgb(200, 100, 100)
                        };
                        ui.label(
                            egui::RichText::new(format!("dogruluk: {:.1}%", metrics.dogruluk_yuzde))
                                .family(egui::FontFamily::Name("spacemono".into()))
                                .size(14.0)
                                .color(dogruluk_rengi),
                        );
                        ui.label(theme::body_text(&format!("dogru: {} | yanlis: {}", metrics.dogru_tus, metrics.yanlis_tus)));
                        ui.add_space(8.0);
                        ui.label(theme::small_text("ESC ile kapat, tekrar basla"));
                    });
                });
            if !open {
                state.just_finished = false;
                state.last_metrics = None;
            }
        }
    }

    // ESC ile sonucu kapat ve yeniden baslat
    ui.ctx().input(|i| {
        if i.key_pressed(egui::Key::Escape) && state.just_finished {
            state.just_finished = false;
            state.last_metrics = None;
        }
    });

    ui.add_space(4.0);

    // === YAZI ALANI ===
    let total_w = ui.available_width();
    let kw = total_w * 0.6;
    let side_pad = (total_w - kw) / 2.0;

    let (next_char, dim_hand) = if state.started {
        let engine = state.engine.as_ref().unwrap();
        let nc = engine.current_word().chars().nth(engine.cursor_in_word);
        let dh = match state.mode {
            TestMode::RightHand => Some(keyboard_widget::Hand::Left),
            TestMode::LeftHand => Some(keyboard_widget::Hand::Right),
            _ => None,
        };
        (nc, dh)
    } else {
        (None, None)
    };

    // === KLAVYE GIRISI ===
    let mut input_chars: Vec<char> = Vec::new();
    let mut space_pressed = false;
    let mut text_char: Option<char> = None;
    let mut any_key_pressed = false;
    let mut any_key_released = false;
    ui.ctx().input(|i| {
        for event in &i.events {
            match event {
                egui::Event::Text(text) => {
                    for ch in text.chars() {
                        if !ch.is_control() && ch != ' ' {
                            input_chars.push(ch);
                            text_char = Some(ch);
                            any_key_pressed = true;
                        }
                    }
                }
                egui::Event::Key { key, pressed, repeat, .. } => {
                    if *pressed && !*repeat && *key == egui::Key::Space {
                        space_pressed = true;
                        text_char = Some(' ');
                        any_key_pressed = true;
                    } else if *pressed && !*repeat {
                        any_key_pressed = true;
                    } else if !*pressed {
                        any_key_released = true;
                    }
                }
                _ => {}
            }
        }
    });

    // pressed_key güncelle: basıliyken karakter göster, bırakınca temizle
    if let Some(ch) = text_char {
        state.pressed_key = Some(ch);
    } else if any_key_released && !any_key_pressed {
        state.pressed_key = None;
    }

    let kb_active_key = if state.show_next_key {
        next_char
    } else {
        state.pressed_key
    };
    ui.ctx().request_repaint();

    ui.add_space(32.0);

    ui.vertical_centered(|ui| {
        ui.set_min_width(kw);
        ui.set_max_width(kw);

        show_typing_area(ui, state, kw);
    });

    // Klavye: en alta sabit, tam ortada
    let kb_height = 52.0 * 5.0 + 4.0 * 4.0 + 16.0;
    let remaining = ui.available_height();
    let bottom_pad = side_pad / 2.0;
    let bottom_space = (remaining - kb_height - bottom_pad).max(0.0);
    ui.add_space(bottom_space);

    let (kb_alloc, _) = ui.allocate_exact_size(egui::vec2(total_w, kb_height), egui::Sense::hover());
    let centered_kb_rect = egui::Rect::from_min_size(
        kb_alloc.min + egui::vec2(side_pad, 0.0),
        egui::vec2(kw, kb_height),
    );
    ui.allocate_new_ui(egui::UiBuilder::new().max_rect(centered_kb_rect), |ui| {
        keyboard_widget::draw_keyboard_sized(ui, kb_active_key, dim_hand, Some(kw));
    });
    ui.add_space(bottom_pad);

    // Her frame'de kelime doldur
    if state.started {
        if let Some(ref mut engine) = state.engine {
            if engine.needs_more_words() {
                if let Some(ref mut ws) = state.word_source {
                    let new_words: Vec<String> = (0..100).map(|_| ws.next_word()).collect();
                    engine.words.extend(new_words);
                    for w in engine.words[engine.typed_words.len()..].iter().cloned() {
                        let char_count = w.chars().count();
                        engine.typed_words.push(crate::typing::engine::TypedWord {
                            word: w,
                            typed_chars: vec![None; char_count],
                            completed: false,
                        });
                    }
                }
            }
        }
    }

    // Harf girisi varsa
    if !input_chars.is_empty() || space_pressed {
        if let Some(s) = sound {
            if space_pressed && input_chars.is_empty() {
                s.play_space();
            } else {
                s.play_key();
            }
        }
        // Bitmisse — ayni kelimelerle devam et (yeni atama)
        if state.just_finished {
            if let Some(ref mut engine) = state.engine {
                engine.reset_for_restart();
            }
            state.started = true;
            state.just_finished = false;
            state.last_metrics = None;
        }
        // Hic baslamadiysa yeni test baslat
        else if !state.started {
            state.started = true;
        }

        let engine = state.engine.as_mut().unwrap();

        // Daha fazla kelime lazimsa yukle
        if engine.needs_more_words() {
            if let Some(ref mut ws) = state.word_source {
                let new_words: Vec<String> = (0..100).map(|_| ws.next_word()).collect();
                engine.words.extend(new_words);
                for w in engine.words[engine.typed_words.len()..].iter().cloned() {
                    let char_count = w.chars().count();
                    engine.typed_words.push(crate::typing::engine::TypedWord {
                        word: w,
                        typed_chars: vec![None; char_count],
                        completed: false,
                    });
                }
            }
        }

        for ch in input_chars {
            engine.on_key(ch);
        }
        if space_pressed {
            engine.on_space();
        }

        if engine.is_finished() {
            let metrics = Metrics::hesapla(engine.dogru_tus, engine.yanlis_tus, engine.elapsed_secs());
            state.last_metrics = Some(metrics);
            state.just_finished = true;
            state.started = false;
        }

        ui.ctx().request_repaint();
    } else if state.started {
        ui.ctx().request_repaint();
    }
}

fn init_engine(state: &mut TestState, json_data: &str) {
    let hand = match state.mode {
        TestMode::RightHand => Some(Hand::Right),
        TestMode::LeftHand => Some(Hand::Left),
        _ => None,
    };

    let mut ws = WordSource::from_json(json_data);
    ws.filter(hand, state.zorluk.as_str());

    let total = state.kelime_hedef;
    let sure = if state.kelime_hedef.is_some() { 0 } else { state.sure_secenek };

    let mut engine = TypingEngine::new(total, sure);
    let initial_words: Vec<String> = (0..200).map(|_| ws.next_word()).collect();
    engine.load_words(initial_words);

    state.engine = Some(engine);
    state.word_source = Some(ws);

    state.test_modu_str = match state.mode {
        TestMode::TwoHand => "iki_el",
        TestMode::RightHand => "sag_el",
        TestMode::LeftHand => "sol_el",
    }
    .to_string();
}

fn reload_engine(state: &mut TestState, json_data: &str) {
    state.engine = None;
    state.word_source = None;
    state.started = false;
    state.just_finished = false;
    state.last_metrics = None;
    init_engine(state, json_data);
}

fn show_typing_area(ui: &mut egui::Ui, state: &mut TestState, container_w: f32) {
    let engine = state.engine.as_ref().unwrap();

    let inner_margin_w = 20.0_f32;
    let content_w = container_w - inner_margin_w * 2.0;
    let line_height = 30.0_f32;
    let padding_top_bottom = 24.0_f32;
    let max_lines = 6usize;
    let total_height = line_height * max_lines as f32 + padding_top_bottom;

    let (alloc_rect, _) = ui.allocate_exact_size(
        egui::vec2(container_w, total_height),
        egui::Sense::hover(),
    );

    let painter = ui.painter();
    painter.rect_filled(
        alloc_rect,
        egui::CornerRadius::same(6),
        theme::DARK_GRAY,
    );
    painter.rect_stroke(
        alloc_rect,
        egui::CornerRadius::same(6),
        egui::Stroke::new(1.0_f32, theme::GRAY),
        egui::StrokeKind::Inside,
    );

    let content_rect = egui::Rect::from_min_size(
        alloc_rect.min + egui::vec2(inner_margin_w, 12.0),
        egui::vec2(content_w, total_height - 24.0),
    );

    ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
        ui.set_clip_rect(content_rect);
        ui.set_max_width(content_w);

        let available_width = content_w;
        let space_width = measure_word_width(ui, " ") * 1.5;
        let word_index = engine.word_index;
        let line_height = 30.0_f32;
        let max_lines = 6usize;

        let mut lines: Vec<Vec<usize>> = Vec::new();
        let mut current_line: Vec<usize> = Vec::new();
        let mut current_width = 0.0_f32;

        for i in word_index..engine.typed_words.len().min(word_index + 80) {
            let tw = &engine.typed_words[i];
            let word_pixel_width = measure_word_width(ui, &tw.word);

            if i > word_index {
                let needed = if current_line.is_empty() {
                    word_pixel_width
                } else {
                    current_width + space_width + word_pixel_width
                };
                if needed > available_width && !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = Vec::new();
                    current_width = 0.0;
                    if lines.len() >= max_lines {
                        break;
                    }
                }
            }

            current_line.push(i);
            if current_line.len() == 1 {
                current_width = word_pixel_width;
            } else {
                current_width += space_width + word_pixel_width;
            }
        }
        if !current_line.is_empty() && lines.len() < max_lines {
            lines.push(current_line);
        }

        let painter = ui.painter().clone();
        let mut y_offset = 0.0_f32;

        for line in lines.iter() {
            let mut x_offset = 0.0_f32;
            for (pos, &i) in line.iter().enumerate() {
                let tw = &engine.typed_words[i];

                if pos > 0 {
                    x_offset += space_width;
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
                        egui::Color32::from_rgb(140, 140, 140)
                    } else {
                        egui::Color32::from_rgb(70, 70, 70)
                    };

                    let text = ch.to_string();
                    let font_id = egui::FontId::new(24.0, egui::FontFamily::Name("spacemono".into()));
                    let galley = ui.fonts(|f| f.layout_no_wrap(text, font_id, color));
                    let gw = galley.size().x;
                    let pos = content_rect.min + egui::vec2(x_offset, y_offset);
                    painter.galley(pos, galley, color);
                    x_offset += gw;
                }
            }
            y_offset += line_height;
        }
    });
}

fn show_results_inline(ui: &mut egui::Ui, metrics: &Metrics, modu: &str, sure_sn: u64) {
    let frame = egui::Frame::NONE
    .fill(theme::DARK_GRAY)
    .corner_radius(egui::CornerRadius::same(6))
    .inner_margin(egui::Margin::same(12))
    .stroke(egui::Stroke::new(1.0_f32, theme::GRAY));

    frame.show(ui, |ui| {
        ui.label(theme::sub_heading_text("sonuclar"));
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(theme::body_text(&format!("mod: {} | sure: {}sn", modu, sure_sn)));
        });
        ui.horizontal_wrapped(|ui| {
            ui.label(theme::body_text(&format!("wpm: {:.1}", metrics.wpm)));
            ui.add_space(12.0);
            ui.label(theme::body_text(&format!("cpm: {:.1}", metrics.cpm)));
            ui.add_space(12.0);

            let dogruluk_rengi = if metrics.dogruluk_yuzde >= 90.0 {
                theme::WHITE
            } else if metrics.dogruluk_yuzde >= 70.0 {
                theme::LIGHT_GRAY
            } else {
                egui::Color32::from_rgb(200, 100, 100)
            };
            ui.label(
                egui::RichText::new(format!("dogruluk: {:.1}%", metrics.dogruluk_yuzde))
                .family(egui::FontFamily::Name("spacemono".into()))
                .size(14.0)
                .color(dogruluk_rengi),
            );
            ui.add_space(12.0);
            ui.label(theme::body_text(&format!("dogru: {} | yanlis: {}", metrics.dogru_tus, metrics.yanlis_tus)));
        });
    });
}

fn show_history_window(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("gecmis")
    .open(open)
    .resizable(true)
    .default_width(500.0)
    .default_height(400.0)
    .show(ctx, |ui| {
        let db = Database::open();
        let results = db.getir_tum();

        if results.is_empty() {
            ui.label(theme::body_text("henuz test sonucu yok"));
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Gruplama
            let mut iki_el: Vec<&crate::storage::models::TestResult> = Vec::new();
            let mut sag_el: Vec<&crate::storage::models::TestResult> = Vec::new();
            let mut sol_el: Vec<&crate::storage::models::TestResult> = Vec::new();

            for r in &results {
                match r.modu.as_str() {
                    "sag_el" => sag_el.push(r),
                                          "sol_el" => sol_el.push(r),
                                          _ => iki_el.push(r),
                }
            }

            for (label, group) in [("iki el", &iki_el), ("sag el", &sag_el), ("sol el", &sol_el)] {
                if group.is_empty() { continue; }
                ui.label(theme::sub_heading_text(label));
                ui.add_space(4.0);

                let count = group.len();
                let avg_wpm: f64 = group.iter().map(|r| r.wpm).sum::<f64>() / count as f64;
                let avg_dog: f64 = group.iter().map(|r| r.dogruluk_yuzde).sum::<f64>() / count as f64;
                let best_wpm = group.iter().map(|r| r.wpm).fold(0.0_f64, f64::max);

                ui.horizontal_wrapped(|ui| {
                    ui.label(theme::body_text(&format!(
                        "test:{} ort.wpm:{:.1} en.iyi:{:.1} dogruluk:{:.1}%",
                        count, avg_wpm, best_wpm, avg_dog
                    )));
                });
    ui.add_space(6.0);
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            egui::Grid::new("history_grid")
            .striped(true)
            .num_columns(6)
            .show(ui, |ui| {
                ui.label(theme::small_text("tarih"));
                ui.label(theme::small_text("mod"));
                ui.label(theme::small_text("wpm"));
                ui.label(theme::small_text("cpm"));
                ui.label(theme::small_text("dogruluk"));
                ui.label(theme::small_text("zorluk"));
                ui.end_row();

                for r in results.iter().rev() {
                    ui.label(theme::body_text(&r.tarih));
                    ui.label(theme::body_text(&r.modu));
                    ui.label(theme::body_text(&format!("{:.1}", r.wpm)));
                    ui.label(theme::body_text(&format!("{:.1}", r.cpm)));
                    ui.label(theme::body_text(&format!("{:.1}%", r.dogruluk_yuzde)));
                    ui.label(theme::body_text(&r.zorluk));
                    ui.end_row();
                }
            });
        });
    });
}

fn measure_word_width(ui: &egui::Ui, word: &str) -> f32 {
    let font_id = egui::FontId::new(24.0, egui::FontFamily::Name("spacemono".into()));
    let galley = ui.fonts(|f| f.layout_no_wrap(word.to_string(), font_id, egui::Color32::WHITE));
    galley.size().x
}
