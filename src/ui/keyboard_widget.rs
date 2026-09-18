use eframe::egui;
use std::collections::HashMap;
use super::theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
    Thumb,
}

impl Finger {
    pub fn color(&self) -> egui::Color32 {
        match self {
            Finger::LeftPinky => egui::Color32::from_rgb(220, 50, 50),
            Finger::LeftRing => egui::Color32::from_rgb(230, 130, 30),
            Finger::LeftMiddle => egui::Color32::from_rgb(230, 200, 30),
            Finger::LeftIndex => egui::Color32::from_rgb(50, 180, 50),
            Finger::RightIndex => egui::Color32::from_rgb(30, 180, 200),
            Finger::RightMiddle => egui::Color32::from_rgb(50, 100, 220),
            Finger::RightRing => egui::Color32::from_rgb(130, 50, 200),
            Finger::RightPinky => egui::Color32::from_rgb(180, 50, 180),
            Finger::Thumb => egui::Color32::from_rgb(150, 150, 150),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Finger::LeftPinky => "sol serce",
            Finger::LeftRing => "sol yuzuk",
            Finger::LeftMiddle => "sol orta",
            Finger::LeftIndex => "sol isaret",
            Finger::RightIndex => "sag isaret",
            Finger::RightMiddle => "sag orta",
            Finger::RightRing => "sag yuzuk",
            Finger::RightPinky => "sag serce",
            Finger::Thumb => "basparmak",
        }
    }

    pub fn label_short(&self) -> &'static str {
        match self {
            Finger::LeftPinky => "L5",
            Finger::LeftRing => "L4",
            Finger::LeftMiddle => "L3",
            Finger::LeftIndex => "L2",
            Finger::RightIndex => "R2",
            Finger::RightMiddle => "R3",
            Finger::RightRing => "R4",
            Finger::RightPinky => "R5",
            Finger::Thumb => "TH",
        }
    }
}

pub fn finger_map() -> HashMap<char, Finger> {
    let mut m = HashMap::new();

    // Sayilari
    m.insert('1', Finger::LeftPinky);
    m.insert('2', Finger::LeftRing);
    m.insert('3', Finger::LeftMiddle);
    m.insert('4', Finger::LeftIndex);
    m.insert('5', Finger::LeftIndex);
    m.insert('6', Finger::RightIndex);
    m.insert('7', Finger::RightIndex);
    m.insert('8', Finger::RightMiddle);
    m.insert('9', Finger::RightRing);
    m.insert('0', Finger::RightPinky);

    // Ust siralar (Q W E R T Y U I O P)
    m.insert('q', Finger::LeftPinky);
    m.insert('w', Finger::LeftRing);
    m.insert('e', Finger::LeftMiddle);
    m.insert('r', Finger::LeftIndex);
    m.insert('t', Finger::LeftIndex);
    m.insert('y', Finger::RightIndex);
    m.insert('u', Finger::RightIndex);
    m.insert('i', Finger::RightMiddle);
    m.insert('o', Finger::RightRing);
    m.insert('p', Finger::RightPinky);

    // Ort siralar (A S D F G H J K L)
    m.insert('a', Finger::LeftPinky);
    m.insert('s', Finger::LeftRing);
    m.insert('d', Finger::LeftMiddle);
    m.insert('f', Finger::LeftIndex);
    m.insert('g', Finger::LeftIndex);
    m.insert('h', Finger::RightIndex);
    m.insert('j', Finger::RightIndex);
    m.insert('k', Finger::RightMiddle);
    m.insert('l', Finger::RightRing);

    // Alt siralar (Z X C V B N M)
    m.insert('z', Finger::LeftPinky);
    m.insert('x', Finger::LeftRing);
    m.insert('c', Finger::LeftMiddle);
    m.insert('v', Finger::LeftIndex);
    m.insert('b', Finger::LeftIndex);
    m.insert('n', Finger::RightIndex);
    m.insert('m', Finger::RightIndex);

    // Turkce ozel
    m.insert('ğ', Finger::RightPinky);
    m.insert('ü', Finger::RightPinky);
    m.insert('ş', Finger::RightPinky);
    m.insert('i', Finger::RightMiddle);
    m.insert('ö', Finger::RightRing);
    m.insert('ç', Finger::RightPinky);

    m
}

pub fn finger_of(c: char) -> Finger {
    let map = finger_map();
    *map.get(&c).unwrap_or(&Finger::Thumb)
}

struct KeyDef {
    label: &'static str,
    finger: Finger,
}

fn keyboard_rows() -> Vec<Vec<KeyDef>> {
    vec![
        vec![
            KeyDef { label: "1", finger: Finger::LeftPinky },
            KeyDef { label: "2", finger: Finger::LeftRing },
            KeyDef { label: "3", finger: Finger::LeftMiddle },
            KeyDef { label: "4", finger: Finger::LeftIndex },
            KeyDef { label: "5", finger: Finger::LeftIndex },
            KeyDef { label: "6", finger: Finger::RightIndex },
            KeyDef { label: "7", finger: Finger::RightIndex },
            KeyDef { label: "8", finger: Finger::RightMiddle },
            KeyDef { label: "9", finger: Finger::RightRing },
            KeyDef { label: "0", finger: Finger::RightPinky },
        ],
        vec![
            KeyDef { label: "Q", finger: Finger::LeftPinky },
            KeyDef { label: "W", finger: Finger::LeftRing },
            KeyDef { label: "E", finger: Finger::LeftMiddle },
            KeyDef { label: "R", finger: Finger::LeftIndex },
            KeyDef { label: "T", finger: Finger::LeftIndex },
            KeyDef { label: "Y", finger: Finger::RightIndex },
            KeyDef { label: "U", finger: Finger::RightIndex },
            KeyDef { label: "I", finger: Finger::RightMiddle },
            KeyDef { label: "O", finger: Finger::RightRing },
            KeyDef { label: "P", finger: Finger::RightPinky },
            KeyDef { label: "Ğ", finger: Finger::RightPinky },
            KeyDef { label: "Ü", finger: Finger::RightPinky },
        ],
        vec![
            KeyDef { label: "A", finger: Finger::LeftPinky },
            KeyDef { label: "S", finger: Finger::LeftRing },
            KeyDef { label: "D", finger: Finger::LeftMiddle },
            KeyDef { label: "F", finger: Finger::LeftIndex },
            KeyDef { label: "G", finger: Finger::LeftIndex },
            KeyDef { label: "H", finger: Finger::RightIndex },
            KeyDef { label: "J", finger: Finger::RightIndex },
            KeyDef { label: "K", finger: Finger::RightMiddle },
            KeyDef { label: "L", finger: Finger::RightRing },
            KeyDef { label: "Ş", finger: Finger::RightPinky },
            KeyDef { label: "İ", finger: Finger::RightPinky },
        ],
        vec![
            KeyDef { label: "Z", finger: Finger::LeftPinky },
            KeyDef { label: "X", finger: Finger::LeftRing },
            KeyDef { label: "C", finger: Finger::LeftMiddle },
            KeyDef { label: "V", finger: Finger::LeftIndex },
            KeyDef { label: "B", finger: Finger::LeftIndex },
            KeyDef { label: "N", finger: Finger::RightIndex },
            KeyDef { label: "M", finger: Finger::RightIndex },
            KeyDef { label: "Ö", finger: Finger::RightRing },
            KeyDef { label: "Ç", finger: Finger::RightPinky },
        ],
    ]
}

pub struct KeyboardWidget {
    pressed_keys: Vec<String>,
    highlight_key: Option<char>,
}

impl Default for KeyboardWidget {
    fn default() -> Self {
        Self {
            pressed_keys: Vec::new(),
            highlight_key: None,
        }
    }
}

impl KeyboardWidget {
    pub fn set_highlight(&mut self, key: Option<char>) {
        self.highlight_key = key;
    }

    pub fn add_press(&mut self, key: String) {
        self.pressed_keys.push(key);
        if self.pressed_keys.len() > 3 {
            self.pressed_keys.remove(0);
        }
    }

    pub fn clear_presses(&mut self) {
        self.pressed_keys.clear();
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        let rows = keyboard_rows();
        let key_width = 32.0;
        let key_height = 32.0;
        let gap = 3.0;

        ui.vertical(|ui| {
            for (row_idx, row) in rows.iter().enumerate() {
                let indent = match row_idx {
                    0 => 0.0,
                    1 => 0.0,
                    2 => key_width * 0.5,
                    3 => key_width * 1.2,
                    _ => 0.0,
                };
                ui.horizontal(|ui| {
                    ui.add_space(indent);
                    for key_def in row {
                        let ch = key_def.label.chars().next().unwrap();
                        let is_highlighted = self.highlight_key.map(|h| {
                            h.to_ascii_lowercase() == ch.to_ascii_lowercase()
                        }).unwrap_or(false);

                        let is_recently_pressed = self.pressed_keys.iter().any(|pk| {
                            pk.chars().next().map(|c| c.to_ascii_lowercase() == ch.to_ascii_lowercase()).unwrap_or(false)
                        });

                        let base_color = key_def.finger.color();
                        let bg = if is_highlighted {
                            egui::Color32::from_rgb(
                                (base_color.r() as u32 * 2 / 3 + 85) as u8,
                                (base_color.g() as u32 * 2 / 3 + 85) as u8,
                                (base_color.b() as u32 * 2 / 3 + 85) as u8,
                            )
                        } else if is_recently_pressed {
                            egui::Color32::from_rgb(255, 255, 255)
                        } else {
                            base_color
                        };

                        let text_color = if is_recently_pressed {
                            theme::BLACK
                        } else {
                            theme::WHITE
                        };

                        let (rect, _response) = ui.allocate_exact_size(
                            egui::vec2(key_width, key_height),
                            egui::Sense::hover(),
                        );

                        let painter = ui.painter();
                        painter.rect_filled(rect, egui::CornerRadius::same(4), bg);
                        painter.rect_stroke(
                            rect,
                            egui::CornerRadius::same(4),
                            egui::Stroke::new(1.0_f32, theme::GRAY),
                            egui::StrokeKind::Inside,
                        );

                        let text = egui::RichText::new(key_def.label)
                            .family(egui::FontFamily::Name("spacemono".into()))
                            .size(12.0)
                            .color(text_color);
                        painter.text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            key_def.label,
                            egui::FontId::proportional(12.0),
                            text_color,
                        );

                        ui.add_space(gap);
                    }
                });
                ui.add_space(gap);
            }

            // Space bar
            ui.horizontal(|ui| {
                ui.add_space(key_width * 1.2);
                let (rect, _response) = ui.allocate_exact_size(
                    egui::vec2(key_width * 6.0, key_height),
                    egui::Sense::hover(),
                );
                let painter = ui.painter();
                let bg = if self.highlight_key == Some(' ') {
                    egui::Color32::from_rgb(120, 120, 120)
                } else {
                    theme::DARK_GRAY
                };
                painter.rect_filled(rect, egui::CornerRadius::same(4), bg);
                painter.rect_stroke(
                    rect,
                    egui::CornerRadius::same(4),
                    egui::Stroke::new(1.0_f32, theme::GRAY),
                    egui::StrokeKind::Inside,
                );
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "SPACE",
                    egui::FontId::proportional(11.0),
                    theme::GRAY,
                );
            });
        });
    }
}

pub fn show_legend(ui: &mut egui::Ui) {
    let fingers = [
        Finger::LeftPinky,
        Finger::LeftRing,
        Finger::LeftMiddle,
        Finger::LeftIndex,
        Finger::RightIndex,
        Finger::RightMiddle,
        Finger::RightRing,
        Finger::RightPinky,
    ];

    ui.horizontal_wrapped(|ui| {
        for finger in &fingers {
            let color = finger.color();
            let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, egui::CornerRadius::same(2), color);
            ui.label(
                egui::RichText::new(finger.label_short())
                    .family(egui::FontFamily::Name("spacemono".into()))
                    .size(9.0)
                    .color(theme::GRAY),
            );
            ui.add_space(4.0);
        }
    });
}
