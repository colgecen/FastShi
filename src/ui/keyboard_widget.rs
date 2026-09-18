use eframe::egui;
use std::collections::HashMap;
use std::time::Instant;
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
    m.insert('ğ', Finger::RightPinky);
    m.insert('ü', Finger::RightPinky);
    m.insert('a', Finger::LeftPinky);
    m.insert('s', Finger::LeftRing);
    m.insert('d', Finger::LeftMiddle);
    m.insert('f', Finger::LeftIndex);
    m.insert('g', Finger::LeftIndex);
    m.insert('h', Finger::RightIndex);
    m.insert('j', Finger::RightIndex);
    m.insert('k', Finger::RightMiddle);
    m.insert('l', Finger::RightRing);
    m.insert('ş', Finger::RightPinky);
    m.insert('ı', Finger::RightPinky);
    m.insert('z', Finger::LeftPinky);
    m.insert('x', Finger::LeftRing);
    m.insert('c', Finger::LeftMiddle);
    m.insert('v', Finger::LeftIndex);
    m.insert('b', Finger::LeftIndex);
    m.insert('n', Finger::RightIndex);
    m.insert('m', Finger::RightIndex);
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
    highlight_key: Option<char>,
    presses: Vec<(String, Instant)>,
}

impl Default for KeyboardWidget {
    fn default() -> Self {
        Self {
            highlight_key: None,
            presses: Vec::new(),
        }
    }
}

impl KeyboardWidget {
    pub fn set_highlight(&mut self, key: Option<char>) {
        self.highlight_key = key;
    }

    pub fn add_press(&mut self, key: String) {
        self.presses.push((key, Instant::now()));
    }

    fn is_pressed(&self, label: &str) -> bool {
        self.presses.iter().any(|(k, t)| {
            let same_key = k.to_uppercase() == label.to_uppercase();
            let fresh = t.elapsed().as_millis() < 200;
            same_key && fresh
        })
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        let rows = keyboard_rows();
        let key_width = 34.0;
        let key_height = 34.0;
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

                        let is_pressed_now = self.is_pressed(key_def.label);

                        let base_color = key_def.finger.color();
                        let bg = if is_pressed_now {
                            egui::Color32::from_rgb(255, 255, 255)
                        } else if is_highlighted {
                            egui::Color32::from_rgb(
                                (base_color.r() as u32 * 2 / 3 + 85) as u8,
                                (base_color.g() as u32 * 2 / 3 + 85) as u8,
                                (base_color.b() as u32 * 2 / 3 + 85) as u8,
                            )
                        } else {
                            base_color
                        };

                        let text_color = if is_pressed_now {
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
                let space_pressed = self.is_pressed("SPACE");
                let bg = if space_pressed {
                    egui::Color32::from_rgb(255, 255, 255)
                } else if self.highlight_key == Some(' ') {
                    egui::Color32::from_rgb(120, 120, 120)
                } else {
                    theme::DARK_GRAY
                };
                let text_col = if space_pressed { theme::BLACK } else { theme::GRAY };
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
                    text_col,
                );
            });
        });

        // Eski press'leri temizle (>500ms onceki)
        // Bu show icinde yapilmiyor cunku &self, ama add_press'te cumulative.
        // Borrows sorunu olmamasi icin temizlik disarda yapilmali.
    }

    pub fn cleanup(&mut self) {
        self.presses.retain(|(_, t)| t.elapsed().as_millis() < 500);
    }
}

pub fn show_legend(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.label(
            egui::RichText::new("parmak haritasi")
                .family(egui::FontFamily::Name("spacemono".into()))
                .size(9.0)
                .color(theme::GRAY),
        );
        ui.add_space(2.0);
        let fingers = [
            (Finger::LeftPinky, "serce (sol)"),
            (Finger::LeftRing, "yuzuk (sol)"),
            (Finger::LeftMiddle, "orta (sol)"),
            (Finger::LeftIndex, "isaret (sol)"),
            (Finger::RightIndex, "isaret (sag)"),
            (Finger::RightMiddle, "orta (sag)"),
            (Finger::RightRing, "yuzuk (sag)"),
            (Finger::RightPinky, "serce (sag)"),
        ];
        for (finger, name) in &fingers {
            ui.horizontal(|ui| {
                let color = finger.color();
                let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                ui.painter().rect_filled(rect, egui::CornerRadius::same(2), color);
                ui.label(
                    egui::RichText::new(*name)
                        .family(egui::FontFamily::Name("spacemono".into()))
                        .size(10.0)
                        .color(theme::LIGHT_GRAY),
                );
            });
        }
    });
}
