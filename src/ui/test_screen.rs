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
    pub keyboard: KeyboardWidget,
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
            keyboard: KeyboardWidget::default(),
        }
    }
}

pub fn show(ui: &mut egui::Ui, state: &mut TestState, json_data: &str, finger_image: Option<&egui::TextureHandle>) {
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
        show_setup(ui, state, json_data, finger_image);
    } else {
        show_typing(ui, state, finger_image);
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
    state.keyboard = KeyboardWidget::default();

    state.test_modu_str = match state.mode {
        TestMode::TwoHand => "iki_el",
        TestMode::RightHand => "sag_el",
        TestMode::LeftHand => "sol_el",
    }
    .to_string();
}

fn show_setup(ui: &mut egui::Ui, state: &mut TestState, json_data: &str, finger_image: Option<&egui::TextureHandle>) {
    let content_w = ui.available_width() * 0.6;
    ui.add_space(30.0);

    ui.allocate_ui_with_layout(
        egui::vec2(content_w, 0.0),
        egui::Layout::top_down(egui::Align::Center),
    |ui| {
        ui.add_space(15.0);

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
                    .min_size(egui::vec2(100.0, 32.0));
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
                    .min_size(egui::vec2(80.0, 32.0));
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
                    .min_size(egui::vec2(80.0, 32.0));
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
                    .min_size(egui::vec2(60.0, 32.0));
                if ui.add(btn).clicked() {
                    state.kelime_hedef = Some(k);
                    state.sure_secenek = 0;
                }
            }
        });

        ui.add_space(24.0);

        let start_label = if let Some(k) = state.kelime_hedef {
            format!("{} kelime basla", k)
        } else {
            format!("{} sn basla", state.sure_secenek)
        };

        ui.vertical_centered(|ui| {
            if ui
                .add(
                    egui::Button::new(theme::button_text(&start_label))
                        .min_size(egui::vec2(200.0, 40.0)),
                )
                .clicked()
            {
                start_test(state, json_data);
            }
        });

        ui.add_space(24.0);

        let frame = egui::Frame::NONE
            .fill(theme::DARK_GRAY)
            .corner_radius(egui::CornerRadius::same(6))
            .inner_margin(egui::Margin::same(10))
            .stroke(egui::Stroke::new(1.0_f32, theme::GRAY));

        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(theme::small_text("klavye duzeni — turkce q"));
                    ui.add_space(4.0);
                    state.keyboard.show(ui);
                    ui.add_space(4.0);
                    keyboard_widget::show_legend(ui);
                });

                if let Some(img) = finger_image {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        let max_h = 220.0;
                        let aspect = img.size()[0] as f32 / img.size()[1] as f32;
                        let desired_h = max_h;
                        let desired_w = desired_h * aspect;
                        let sized = egui::load::SizedTexture::new(img.id(), egui::vec2(desired_w, desired_h));
                        ui.image(egui::ImageSource::Texture(sized));
                    });
                }
            });
        });

        ui.add_space(15.0);
    });
}

fn show_typing(ui: &mut egui::Ui, state: &mut TestState, finger_image: Option<&egui::TextureHandle>) {
    let engine = state.engine.as_mut().unwrap();
    let word_source = state.word_source.as_mut().unwrap();

    // Daha fazla kelime lazimsa yukle
    if engine.needs_more_words() {
        let new_words: Vec<String> = (0..30).map(|_| word_source.next_word()).collect();
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

    // Klavye girisini oku
    let mut input_chars: Vec<char> = Vec::new();
    let mut space_pressed = false;
    ui.ctx().input(|i| {
        for event in &i.events {
            match event {
                egui::Event::Text(text) => {
                    for ch in text.chars() {
                        if !ch.is_control() && ch != ' ' {
                            input_chars.push(ch);
                        }
                    }
                }
                egui::Event::Key { key, pressed, repeat, .. } => {
                    if *pressed && !*repeat && *key == egui::Key::Space {
                        space_pressed = true;
                    }
                }
                _ => {}
            }
        }
    });

    // Klavuz tuslari isle
    for ch in input_chars {
        engine.on_key(ch);
        state.keyboard.add_press(ch.to_string().to_uppercase());
    }
    if space_pressed {
        engine.on_space();
        state.keyboard.add_press("SPACE".to_string());
    }
    // Highlight guncelle
    if let Some(next_ch) = engine.current_word().chars().nth(engine.cursor_in_word) {
        state.keyboard.set_highlight(Some(next_ch));
    } else {
        state.keyboard.set_highlight(None);
    }
    state.keyboard.cleanup();

    // === EKRAN YERLESIMI ===
    let content_w = ui.available_width() * 0.6;

    ui.add_space(20.0);

    ui.allocate_ui_with_layout(
        egui::vec2(content_w, 0.0),
        egui::Layout::top_down(egui::Align::Center),
    |ui| {

        // Ust satir: sure + dogruluk
        ui.horizontal(|ui| {
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

        ui.add_space(8.0);

        // === PARAGRAF KUTUSU ===
        let frame = egui::Frame::NONE
            .fill(theme::DARK_GRAY)
            .corner_radius(egui::CornerRadius::same(6))
            .inner_margin(egui::Margin::symmetric(16, 14))
            .stroke(egui::Stroke::new(1.0_f32, theme::GRAY));

        frame.show(ui, |ui| {
            ui.set_min_width(content_w - 32.0);

            let available_width = content_w - 32.0;
            let space_width = 10.0;
            let word_index = engine.word_index;

            // Satir bazli yerlestirme — kelime hicbir zaman yarim kalmaz
            let mut lines: Vec<Vec<usize>> = Vec::new();
            let mut current_line: Vec<usize> = Vec::new();
            let mut current_width = 0.0_f32;

            for i in word_index..engine.typed_words.len().min(word_index + 30) {
                let tw = &engine.typed_words[i];
                // Her harf icin approx genislik hesapla
                let word_pixel_width = measure_word_width(&tw.word);

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
                    }
                }

                current_line.push(i);
                if current_line.len() == 1 {
                    current_width = word_pixel_width;
                } else {
                    current_width += space_width + word_pixel_width;
                }
            }
            if !current_line.is_empty() {
                lines.push(current_line);
            }

            // Her satiri ciz
            for (line_idx, line) in lines.iter().enumerate() {
                ui.horizontal(|ui| {
                    for (pos, &i) in line.iter().enumerate() {
                        let tw = &engine.typed_words[i];

                        // Satir basinda bosluk ekleme (ilk kelime disinda)
                        if pos > 0 {
                            ui.label(
                                egui::RichText::new(" ")
                                    .family(egui::FontFamily::Name("spacemono".into()))
                                    .size(22.0)
                                    .color(theme::GRAY),
                            );
                        }

                        // Kelimeyi harf harf ciz
                        for (ci, ch) in tw.word.chars().enumerate() {
                            let color = if tw.completed {
                                // Tamamlanmis kelime — tamamin beyaz
                                theme::WHITE
                            } else if i == word_index && ci < engine.cursor_in_word {
                                // Dogru yazilmis harfler
                                theme::WHITE
                            } else if i == word_index && ci == engine.cursor_in_word && engine.has_error {
                                // Yanlis harf — kirmizi
                                egui::Color32::from_rgb(255, 80, 80)
                            } else if i == word_index && ci == engine.cursor_in_word {
                                // Aktif harf — parmak rengi
                                let finger = keyboard_widget::finger_of(ch);
                                finger.color()
                            } else if i == word_index {
                                // Yazilmamis ama su anki kelime — acik gri
                                egui::Color32::from_rgb(140, 140, 140)
                            } else {
                                // Diger kelimeler — koyu gri
                                egui::Color32::from_rgb(70, 70, 70)
                            };

                            let rt = egui::RichText::new(ch)
                                .family(egui::FontFamily::Name("spacemono".into()))
                                .size(22.0)
                                .color(color);
                            ui.label(rt);
                        }
                    }
                });
                ui.add_space(2.0);
            }
        });

        ui.add_space(10.0);

        // === KLAVYE ===
        let kb_frame = egui::Frame::NONE
            .fill(theme::DARK_GRAY)
            .corner_radius(egui::CornerRadius::same(6))
            .inner_margin(egui::Margin::symmetric(16, 10))
            .stroke(egui::Stroke::new(1.0_f32, theme::GRAY));

        kb_frame.show(ui, |ui| {
            ui.set_min_width(content_w - 32.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    state.keyboard.show(ui);
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if let Some(img) = finger_image {
                        let max_h = 180.0;
                        let aspect = img.size()[0] as f32 / img.size()[1] as f32;
                        let desired_h = max_h;
                        let desired_w = desired_h * aspect;
                        let sized = egui::load::SizedTexture::new(img.id(), egui::vec2(desired_w, desired_h));
                        ui.image(egui::ImageSource::Texture(sized));
                    }
                    ui.add_space(8.0);
                    keyboard_widget::show_legend(ui);
                });
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

/// Tahmini kelime genisligi (Space Mono 22pt icin harf basina ~13px)
fn measure_word_width(word: &str) -> f32 {
    let char_width = 13.0;
    let mut w = 0.0_f32;
    for ch in word.chars() {
        // Turkce ozel karakterler biraz daha genis
        w += match ch {
            'm' | 'w' | 'M' | 'W' => char_width * 1.5,
            'i' | 'ı' | 'l' | '1' | '!' | '|' => char_width * 0.7,
            _ => char_width,
        };
    }
    w
}
