use eframe::egui;

use crate::hand_mode::key_map::Hand;
use crate::storage::db::Database;
use crate::storage::models::TestResult;
use crate::typing::engine::{TestMode, TypingEngine, Zorluk};
use crate::typing::metrics::Metrics;
use crate::typing::word_source::WordSource;
use crate::ui::keyboard_widget;
use super::theme;

const LAYOUT_MARGIN: f32 = 14.0;
const LAYOUT_GAP: f32 = 14.0;
const TYPING_LINE_HEIGHT: f32 = 30.0;
const TYPING_MAX_LINES: usize = 6;
const TYPING_BOX_HEIGHT: f32 = TYPING_LINE_HEIGHT * TYPING_MAX_LINES as f32 + 24.0;
const FINGER_HEIGHT: f32 = 72.0;
const KEYBOARD_ROW_HEIGHT: f32 = 52.0;
const KEYBOARD_HEIGHT: f32 = KEYBOARD_ROW_HEIGHT * 5.0 + 4.0 * 4.0 + 16.0;
const CENTER_GAP: f32 = 30.0;
const TYPING_WIDTH_RATIO: f32 = 0.60;
const KEYBOARD_WIDTH_FACTOR: f32 = 1.10;

fn keyboard_width(typing_w: f32, center_w: f32) -> f32 {
    (typing_w * KEYBOARD_WIDTH_FACTOR).min(center_w)
}

struct ColumnLayout {
    card_w: f32,
    card_h: f32,
    center_w: f32,
}

fn column_layout(avail: egui::Vec2) -> ColumnLayout {
    let card_w = (avail.x * 0.17).clamp(230.0, 300.0);
    let card_h = (avail.y - LAYOUT_MARGIN * 2.0).max(360.0);
    let center_w =
        (avail.x - card_w * 2.0 - LAYOUT_GAP * 2.0 - LAYOUT_MARGIN * 2.0).max(320.0);

    ColumnLayout {
        card_w,
        card_h,
        center_w,
    }
}

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
    pub show_next_key: bool,
    pub show_finger: bool,
    pub pressed_key: Option<char>,
    pub history: Vec<TestResult>,
    pub history_loaded: bool,
    pub result_saved: bool,
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
            show_next_key: true,
            show_finger: true,
            pressed_key: None,
            history: Vec::new(),
            history_loaded: false,
            result_saved: false,
        }
    }
}

impl TestState {
    pub fn refresh_history(&mut self) {
        self.history = Database::open().getir_tum();
        self.history_loaded = true;
    }
}

pub fn show(
    ui: &mut egui::Ui,
    state: &mut TestState,
    json_data: &str,
    _finger_image: Option<&egui::TextureHandle>,
    _logo_image: Option<&egui::TextureHandle>,
    sound: Option<&crate::sound::SoundManager>,
) {
    // Her zaman kelimeleri yuklu tut — eger engine yoksa baslat
    if state.engine.is_none() {
        init_engine(state, json_data);
    }

    if !state.history_loaded {
        state.refresh_history();
    }

    // Sonuc penceresi
    show_results_window(ui.ctx(), state);

    // ESC ile sonucu kapat ve yeniden baslat
    ui.ctx().input(|i| {
        if i.key_pressed(egui::Key::Escape) && state.just_finished {
            state.just_finished = false;
            state.last_metrics = None;
        }
    });

    // dim_hand her zaman moda göre: sağ el seçince sol kaybolsun, sol seçince sağ kaybolsun, iki elde geri gelsin
    let dim_hand = match state.mode {
        TestMode::RightHand => Some(keyboard_widget::Hand::Left),
        TestMode::LeftHand => Some(keyboard_widget::Hand::Right),
        _ => None,
    };
    let next_char = if state.started {
        let engine = state.engine.as_ref().unwrap();
        let word = engine.current_word();
        let len = word.chars().count();
        if engine.cursor_in_word < len {
            word.chars().nth(engine.cursor_in_word)
        } else {
            // kelime bitti, sıradaki tuş boşluk -> başparmak
            Some(' ')
        }
    } else if let Some(engine) = state.engine.as_ref() {
        // henüz başlamadıysa ilk harfin parmağını göster
        engine.current_word().chars().next()
    } else {
        None
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

    // Tuşa basınca beyaz/karşıt renk: basılı tuş öncelikli, yoksa sıradaki harf
    let kb_active_key = if state.pressed_key.is_some() {
        state.pressed_key
    } else if state.show_next_key {
        next_char
    } else {
        None
    };
    ui.ctx().request_repaint();

    // === YERLESIM: sol ayarlar | orta test alani | sag gecmis ===
    let layout = column_layout(ui.available_size());
    let typing_w = (layout.center_w * TYPING_WIDTH_RATIO).min(layout.center_w);
    let keyboard_w = keyboard_width(typing_w, layout.center_w);

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.add_space(LAYOUT_MARGIN);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(LAYOUT_MARGIN);

            draw_card(ui, egui::vec2(layout.card_w, layout.card_h), |ui| {
                show_settings_panel(ui, state, json_data, sound);
            });

            ui.add_space(LAYOUT_GAP);

            let (center_rect, _) = ui.allocate_exact_size(
                egui::vec2(layout.center_w, layout.card_h),
                egui::Sense::hover(),
            );
            let mut center_ui =
                ui.new_child(egui::UiBuilder::new().max_rect(center_rect));
            center_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                show_test_panel(
                    ui,
                    state,
                    layout.center_w,
                    layout.card_h,
                    typing_w,
                    keyboard_w,
                    next_char,
                    kb_active_key,
                    dim_hand,
                );
            });

            ui.add_space(LAYOUT_GAP);

            draw_card(ui, egui::vec2(layout.card_w, layout.card_h), |ui| {
                show_history_panel(ui, state);
            });

            ui.add_space(LAYOUT_MARGIN);
        });
    });

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
            state.result_saved = false;
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
            state.result_saved = false;
        }

        ui.ctx().request_repaint();
    } else if state.started {
        ui.ctx().request_repaint();
    }
}

fn draw_card(ui: &mut egui::Ui, size: egui::Vec2, add: impl FnOnce(&mut egui::Ui)) {
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let radius = egui::CornerRadius::same(10);

    ui.painter()
        .rect_filled(rect, radius, theme::DARK_GRAY);
    ui.painter().rect_stroke(
        rect,
        radius,
        egui::Stroke::new(1.0_f32, theme::GRAY),
        egui::StrokeKind::Inside,
    );

    let inner = rect.shrink(14.0);
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(inner)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    child.set_clip_rect(inner);
    add(&mut child);
}

fn show_settings_panel(
    ui: &mut egui::Ui,
    state: &mut TestState,
    json_data: &str,
    sound: Option<&crate::sound::SoundManager>,
) {
    egui::ScrollArea::vertical()
        .id_salt("ayarlar_panel")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.label(theme::heading_text("FastShi"));
            ui.add_space(2.0);
            ui.label(theme::small_text("hizli yazma uygulamasi"));
            ui.add_space(22.0);

            ui.label(theme::sub_heading_text("mod"));
            ui.add_space(10.0);
            let btn_w = (ui.available_width() - 12.0) / 3.0;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                for (mode, label) in [
                    (TestMode::TwoHand, "iki el"),
                    (TestMode::RightHand, "sag el"),
                    (TestMode::LeftHand, "sol el"),
                ] {
                    let selected = state.mode == mode;
                    if option_button(ui, label, selected, btn_w).clicked() && state.mode != mode {
                        state.mode = mode;
                        reload_engine(state, json_data);
                    }
                }
            });
            ui.add_space(18.0);

            ui.label(theme::sub_heading_text("zorluk"));
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                for (z, label) in [
                    (Zorluk::Kolay, "kolay"),
                    (Zorluk::Orta, "orta"),
                    (Zorluk::Zor, "zor"),
                ] {
                    let selected = state.zorluk == z;
                    if option_button(ui, label, selected, btn_w).clicked() && state.zorluk != z {
                        state.zorluk = z;
                        reload_engine(state, json_data);
                    }
                }
            });
            ui.add_space(18.0);

            ui.label(theme::sub_heading_text("sure"));
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                for sn in [30, 60, 120] {
                    let selected = state.sure_secenek == sn && state.kelime_hedef.is_none();
                    let label = format!("{}sn", sn);
                    if option_button(ui, &label, selected, btn_w).clicked() {
                        let needs_reload =
                            state.sure_secenek != sn || state.kelime_hedef.is_some();
                        state.sure_secenek = sn;
                        state.kelime_hedef = None;
                        if needs_reload {
                            reload_engine(state, json_data);
                        }
                    }
                }
            });
            ui.add_space(18.0);

            ui.label(theme::sub_heading_text("kelime"));
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                for k in [25, 50, 100] {
                    let selected = state.kelime_hedef == Some(k);
                    let label = format!("{}", k);
                    if option_button(ui, &label, selected, btn_w).clicked() {
                        let needs_reload =
                            state.kelime_hedef != Some(k) || state.sure_secenek != 0;
                        state.kelime_hedef = Some(k);
                        state.sure_secenek = 0;
                        if needs_reload {
                            reload_engine(state, json_data);
                        }
                    }
                }
            });

            ui.add_space(22.0);
            ui.separator();
            ui.add_space(16.0);

            let full_w = ui.available_width();
            let harf_label = if state.show_next_key {
                "harf: acik"
            } else {
                "harf: kapali"
            };
            if option_button(ui, harf_label, state.show_next_key, full_w).clicked() {
                state.show_next_key = !state.show_next_key;
            }
            ui.add_space(8.0);

            let parmak_label = if state.show_finger {
                "parmak: acik"
            } else {
                "parmak: kapali"
            };
            if option_button(ui, parmak_label, state.show_finger, full_w).clicked() {
                state.show_finger = !state.show_finger;
            }

            ui.add_space(20.0);
            ui.label(theme::sub_heading_text("ses"));
            ui.add_space(10.0);
            if let Some(snd) = sound {
                let avail = ui.available_width();
                let bar_w = (avail * 0.8).clamp(140.0, 220.0);
                let side_pad = ((avail - bar_w) / 2.0).max(0.0);

                ui.horizontal(|ui| {
                    ui.add_space(side_pad);
                    ui.spacing_mut().slider_width = bar_w;
                    let mut vol = snd.volume();
                    let slider = egui::Slider::new(&mut vol, 0.0..=1.0)
                        .show_value(false)
                        .trailing_fill(true);
                    if ui.add(slider).changed() {
                        snd.set_volume(vol);
                    }
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(side_pad);
                    let icon = if snd.is_muted() {
                        "🔇"
                    } else if snd.volume() < 0.5 {
                        "🔈"
                    } else {
                        "🔊"
                    };
                    if ui
                        .add_sized(egui::vec2(44.0, 28.0), egui::Button::new(icon))
                        .clicked()
                    {
                        snd.toggle_mute();
                    }
                    ui.add_space(10.0);
                    ui.label(theme::small_text(&format!(
                        "%{}",
                        (snd.volume() * 100.0) as i32
                    )));
                });
            } else {
                ui.label(theme::small_text("ses kapali"));
            }
        });
}

fn option_button(
    ui: &mut egui::Ui,
    label: &str,
    selected: bool,
    width: f32,
) -> egui::Response {
    let btn = egui::Button::new(theme::button_text(label))
        .selected(selected)
        .min_size(egui::vec2(width, 28.0));
    ui.add(btn)
}

fn show_test_panel(
    ui: &mut egui::Ui,
    state: &mut TestState,
    center_w: f32,
    center_h: f32,
    typing_w: f32,
    keyboard_w: f32,
    next_char: Option<char>,
    kb_active_key: Option<char>,
    dim_hand: Option<keyboard_widget::Hand>,
) {
    let status_h = 28.0;
    let (status_rect, _) =
        ui.allocate_exact_size(egui::vec2(center_w, status_h), egui::Sense::hover());
    let mut status_ui = ui.new_child(egui::UiBuilder::new().max_rect(status_rect));
    status_ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
                if state.started {
                    let engine = state.engine.as_ref().unwrap();
                    let elapsed = engine.elapsed_secs();
                    if state.sure_secenek > 0 {
                        let remaining = state.sure_secenek as f64 - elapsed;
                        ui.label(theme::body_text(&format!(
                            "{:.0} sn kaldi",
                            remaining.max(0.0)
                        )));
                    } else {
                        ui.label(theme::body_text(&format!(
                            "{}/{} kelime",
                            engine.words_completed,
                            state.kelime_hedef.unwrap_or(0)
                        )));
                    }
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            let toplam = engine.dogru_tus + engine.yanlis_tus;
                            let dogruluk = if toplam > 0 {
                                (engine.dogru_tus as f64 / toplam as f64 * 100.0) as u32
                            } else {
                                100
                            };
                            ui.label(theme::body_text(&format!("%{}", dogruluk)));
                        },
                    );
                } else if state.just_finished {
                    ui.label(theme::small_text("ESC ile sonucu kapat"));
                } else {
                    ui.label(theme::small_text("yazmaya baslamak icin bir tusa bas"));
                }
            },
        );

    let group_h =
        TYPING_BOX_HEIGHT + CENTER_GAP * 2.0 + FINGER_HEIGHT + KEYBOARD_HEIGHT;
    let free = (center_h - status_h - group_h).max(0.0);
    ui.add_space(free / 2.0);

    show_typing_area(ui, state, typing_w);
    ui.add_space(CENTER_GAP);

    let finger_text = if state.show_finger {
        if let Some(ch) = next_char {
            keyboard_widget::finger_of(ch).display_name().to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let (finger_rect, _) =
        ui.allocate_exact_size(egui::vec2(center_w, FINGER_HEIGHT), egui::Sense::hover());
    if !finger_text.is_empty() {
        let mut finger_ui = ui.new_child(egui::UiBuilder::new().max_rect(finger_rect));
        finger_ui.centered_and_justified(|ui| {
            ui.label(
                egui::RichText::new(finger_text)
                    .family(egui::FontFamily::Name("rajdhani_bold".into()))
                    .size(64.0)
                    .color(theme::WHITE),
            );
        });
    }

    ui.add_space(CENTER_GAP);
    keyboard_widget::draw_keyboard_sized(ui, kb_active_key, dim_hand, Some(keyboard_w));
}

fn show_history_panel(ui: &mut egui::Ui, state: &TestState) {
    ui.label(theme::heading_text("gecmis"));
    ui.add_space(2.0);
    ui.label(theme::small_text(&format!("{} kayit", state.history.len())));
    ui.add_space(14.0);

    if state.history.is_empty() {
        ui.label(theme::body_text("henuz test sonucu yok"));
        return;
    }

    let mut iki_el: Vec<&TestResult> = Vec::new();
    let mut sag_el: Vec<&TestResult> = Vec::new();
    let mut sol_el: Vec<&TestResult> = Vec::new();

    for r in &state.history {
        match r.modu.as_str() {
            "sag_el" => sag_el.push(r),
            "sol_el" => sol_el.push(r),
            _ => iki_el.push(r),
        }
    }

    for (label, group) in [
        ("iki el", &iki_el),
        ("sag el", &sag_el),
        ("sol el", &sol_el),
    ] {
        if group.is_empty() {
            continue;
        }

        let count = group.len();
        let avg_wpm: f64 = group.iter().map(|r| r.wpm).sum::<f64>() / count as f64;
        let avg_dog: f64 =
            group.iter().map(|r| r.dogruluk_yuzde).sum::<f64>() / count as f64;
        let best_wpm = group.iter().map(|r| r.wpm).fold(0.0_f64, f64::max);

        ui.label(theme::sub_heading_text(label));
        ui.add_space(3.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            ui.label(theme::small_text(&format!("test: {}", count)));
            ui.label(theme::small_text(&format!("ort.wpm: {:.1}", avg_wpm)));
        });
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            ui.label(theme::small_text(&format!("en iyi: {:.1}", best_wpm)));
            ui.label(theme::small_text(&format!("dog: {:.1}%", avg_dog)));
        });
        ui.add_space(8.0);
    }

    ui.separator();
    ui.add_space(8.0);

    egui::ScrollArea::vertical()
        .id_salt("gecmis_panel")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (i, r) in state.history.iter().enumerate() {
                if i > 0 {
                    ui.add_space(8.0);
                }
                show_result_entry(ui, r);
            }
        });
}

fn show_result_entry(ui: &mut egui::Ui, r: &TestResult) {
    ui.horizontal(|ui| {
        ui.label(theme::small_text(&r.tarih));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(theme::small_text(&format!("%{:.0}", r.dogruluk_yuzde)));
        });
    });
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        ui.label(theme::body_text(&format!("{:.1} wpm", r.wpm)));
        ui.label(theme::small_text(&format!("{:.0} cpm", r.cpm)));
    });
    ui.label(theme::small_text(&format!("{} · {}", r.modu, r.zorluk)));
}

fn show_results_window(ctx: &egui::Context, state: &mut TestState) {
    if !state.just_finished {
        return;
    }
    let Some(metrics) = state.last_metrics.clone() else {
        return;
    };

    let mut open = true;
    egui::Window::new("sonuclar")
        .open(&mut open)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .fixed_size([320.0, 200.0])
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.label(theme::sub_heading_text(&format!(
                    "mod: {}",
                    state.test_modu_str
                )));
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
                    egui::RichText::new(format!(
                        "dogruluk: {:.1}%",
                        metrics.dogruluk_yuzde
                    ))
                    .family(egui::FontFamily::Name("spacemono".into()))
                    .size(14.0)
                    .color(dogruluk_rengi),
                );
                ui.label(theme::body_text(&format!(
                    "dogru: {} | yanlis: {}",
                    metrics.dogru_tus, metrics.yanlis_tus
                )));
                ui.add_space(8.0);
                ui.label(theme::small_text("ESC ile kapat, tekrar basla"));
            });
        });

    if !open {
        state.just_finished = false;
        state.last_metrics = None;
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
    state.result_saved = false;
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

    let mut content_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));
    content_ui.set_clip_rect(content_rect);
    content_ui.set_max_width(content_w);
    {
        let ui = &mut content_ui;
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
    }
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

fn measure_word_width(ui: &egui::Ui, word: &str) -> f32 {
    let font_id = egui::FontId::new(24.0, egui::FontFamily::Name("spacemono".into()));
    let galley = ui.fonts(|f| f.layout_no_wrap(word.to_string(), font_id, egui::Color32::WHITE));
    galley.size().x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uc_sutun_duzeni_esit_ve_ortali() {
        let layout = column_layout(egui::vec2(1920.0, 1080.0));
        let total = layout.card_w * 2.0
            + layout.center_w
            + LAYOUT_GAP * 2.0
            + LAYOUT_MARGIN * 2.0;

        assert!((total - 1920.0).abs() < 0.01);
        assert!(layout.card_w > 0.0);
        assert!(layout.card_h > 0.0);
        assert!(layout.center_w > 0.0);
    }

    #[test]
    fn klavye_yazi_alanindan_yuzde_on_genis() {
        assert!((keyboard_width(600.0, 2000.0) - 660.0).abs() < 0.01);
        assert!((keyboard_width(600.0, 620.0) - 620.0).abs() < 0.01);
    }
}

