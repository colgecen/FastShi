use eframe::egui::{Align2, Color32, CornerRadius, FontId, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2};

pub const C_GREEN: Color32 = Color32::from_rgb(172, 237, 16);
pub const C_LAVENDER: Color32 = Color32::from_rgb(192, 211, 255);
pub const C_CREAM: Color32 = Color32::from_rgb(252, 247, 197);
pub const C_RED: Color32 = Color32::from_rgb(252, 53, 76);
pub const C_TEAL: Color32 = Color32::from_rgb(10, 191, 188);
pub const C_TAN: Color32 = Color32::from_rgb(219, 156, 92);
pub const C_PINK: Color32 = Color32::from_rgb(237, 192, 236);
pub const C_ORANGE: Color32 = Color32::from_rgb(255, 98, 0);
pub const C_INDIGO: Color32 = Color32::from_rgb(77, 56, 209);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Key {
    pub id: Option<char>,
    pub bottom: &'static str,
    pub top: Option<&'static str>,
    pub color: Color32,
    pub width: f32,
    pub hand: Option<Hand>,
}

const fn k(
    id: Option<char>,
    bottom: &'static str,
    top: Option<&'static str>,
    color: Color32,
    width: f32,
    hand: Option<Hand>,
) -> Key {
    Key { id, bottom, top, color, width, hand }
}

fn row1() -> Vec<Key> {
    vec![
        k(None, "\"", Some("é"), C_GREEN, 1.0, Some(Hand::Left)),
        k(Some('1'), "1", Some("!"), C_GREEN, 1.0, Some(Hand::Left)),
        k(Some('2'), "2", Some("'"), C_LAVENDER, 1.0, Some(Hand::Left)),
        k(Some('3'), "3", Some("^"), C_CREAM, 1.0, Some(Hand::Left)),
        k(Some('4'), "4", Some("+"), C_RED, 1.0, Some(Hand::Left)),
        k(Some('5'), "5", Some("%"), C_RED, 1.0, Some(Hand::Left)),
        k(Some('6'), "6", Some("&"), C_TEAL, 1.0, Some(Hand::Right)),
        k(Some('7'), "7", Some("/"), C_TEAL, 1.0, Some(Hand::Right)),
        k(Some('8'), "8", Some("("), C_TAN, 1.0, Some(Hand::Right)),
        k(Some('9'), "9", Some(")"), C_PINK, 1.0, Some(Hand::Right)),
        k(Some('0'), "0", Some("="), C_ORANGE, 1.0, Some(Hand::Right)),
        k(None, "*", Some("?"), C_ORANGE, 1.0, Some(Hand::Right)),
        k(None, "_", Some("-"), C_ORANGE, 1.0, Some(Hand::Right)),
        k(None, "Delete", None, C_ORANGE, 1.75, Some(Hand::Right)),
    ]
}

fn row2() -> Vec<Key> {
    vec![
        k(None, "Tab", None, C_GREEN, 1.5, Some(Hand::Left)),
        k(Some('q'), "Q", None, C_GREEN, 1.0, Some(Hand::Left)),
        k(Some('w'), "W", None, C_LAVENDER, 1.0, Some(Hand::Left)),
        k(Some('e'), "E", None, C_CREAM, 1.0, Some(Hand::Left)),
        k(Some('r'), "R", None, C_RED, 1.0, Some(Hand::Left)),
        k(Some('t'), "T", None, C_RED, 1.0, Some(Hand::Left)),
        k(Some('y'), "Y", None, C_TEAL, 1.0, Some(Hand::Right)),
        k(Some('u'), "U", None, C_TEAL, 1.0, Some(Hand::Right)),
        k(Some('ı'), "I", None, C_TAN, 1.0, Some(Hand::Right)),
        k(Some('o'), "O", None, C_PINK, 1.0, Some(Hand::Right)),
        k(Some('p'), "P", None, C_ORANGE, 1.0, Some(Hand::Right)),
        k(Some('ğ'), "Ğ", None, C_ORANGE, 1.0, Some(Hand::Right)),
        k(Some('ü'), "Ü", None, C_ORANGE, 1.0, Some(Hand::Right)),
        k(None, "Enter", None, C_ORANGE, 1.75, Some(Hand::Right)),
    ]
}

fn row3() -> Vec<Key> {
    vec![
        k(None, "Caps Lock", None, C_GREEN, 1.75, Some(Hand::Left)),
        k(Some('a'), "A", None, C_GREEN, 1.0, Some(Hand::Left)),
        k(Some('s'), "S", None, C_LAVENDER, 1.0, Some(Hand::Left)),
        k(Some('d'), "D", None, C_CREAM, 1.0, Some(Hand::Left)),
        k(Some('f'), "F", None, C_RED, 1.0, Some(Hand::Left)),
        k(Some('g'), "G", None, C_RED, 1.0, Some(Hand::Left)),
        k(Some('h'), "H", None, C_TEAL, 1.0, Some(Hand::Right)),
        k(Some('j'), "J", None, C_TEAL, 1.0, Some(Hand::Right)),
        k(Some('k'), "K", None, C_TAN, 1.0, Some(Hand::Right)),
        k(Some('l'), "L", None, C_PINK, 1.0, Some(Hand::Right)),
        k(Some('ş'), "Ş", None, C_ORANGE, 1.0, Some(Hand::Right)),
        k(Some('i'), "İ", None, C_ORANGE, 1.0, Some(Hand::Right)),
        k(None, ",", Some(";"), C_ORANGE, 1.0, Some(Hand::Right)),
    ]
}

fn row4() -> Vec<Key> {
    vec![
        k(None, "Shift", None, C_GREEN, 1.75, Some(Hand::Left)),
        k(None, "<", Some(">"), C_GREEN, 1.0, Some(Hand::Left)),
        k(Some('z'), "Z", None, C_GREEN, 1.0, Some(Hand::Left)),
        k(Some('x'), "X", None, C_LAVENDER, 1.0, Some(Hand::Left)),
        k(Some('c'), "C", None, C_CREAM, 1.0, Some(Hand::Left)),
        k(Some('v'), "V", None, C_RED, 1.0, Some(Hand::Left)),
        k(Some('b'), "B", None, C_RED, 1.0, Some(Hand::Left)),
        k(Some('n'), "N", None, C_TEAL, 1.0, Some(Hand::Right)),
        k(Some('m'), "M", None, C_TEAL, 1.0, Some(Hand::Right)),
        k(Some('ö'), "Ö", None, C_TAN, 1.0, Some(Hand::Right)),
        k(Some('ç'), "Ç", None, C_PINK, 1.0, Some(Hand::Right)),
        k(None, ".", Some(":"), C_ORANGE, 1.0, Some(Hand::Right)),
        k(None, "Shift", None, C_ORANGE, 1.75, Some(Hand::Right)),
    ]
}

fn row5() -> Vec<Key> {
    vec![
        k(None, "Ctrl", None, C_GREEN, 1.25, Some(Hand::Left)),
        k(None, "Alt", None, C_GREEN, 1.25, Some(Hand::Left)),
        k(None, "Cmd", None, C_INDIGO, 1.25, Some(Hand::Left)),
        k(Some(' '), "", None, C_INDIGO, 6.5, None),
        k(None, "Cmd", None, C_INDIGO, 1.25, Some(Hand::Right)),
        k(None, "Alt Gr", None, C_PINK, 1.25, Some(Hand::Right)),
        k(None, "Start", None, C_ORANGE, 1.25, Some(Hand::Right)),
        k(None, "Ctrl", None, C_ORANGE, 1.25, Some(Hand::Right)),
    ]
}

pub fn draw_keyboard(ui: &mut Ui, active_key: Option<char>, dim_hand: Option<Hand>) {
    draw_keyboard_sized(ui, active_key, dim_hand, None);
}

pub fn draw_keyboard_sized(ui: &mut Ui, active_key: Option<char>, dim_hand: Option<Hand>, width: Option<f32>) {
    let row_height = 52.0;
    let row_gap = 4.0;
    let key_gap = 3.0;
    let padding = 8.0;

    let rows: [Vec<Key>; 5] = [row1(), row2(), row3(), row4(), row5()];

    let total_height = row_height * 5.0 + row_gap * 4.0 + padding * 2.0;
    let total_width = width.unwrap_or_else(|| ui.available_width());

    let (alloc_rect, _) = ui.allocate_exact_size(
        Vec2::new(total_width, total_height),
        egui::Sense::hover(),
    );

    let origin = Pos2::new(alloc_rect.min.x + padding, alloc_rect.min.y + padding);
    let draw_width = total_width - padding * 2.0;
    let mut y = origin.y;

    // L-şekilli Enter uzantısı tuşları kapatıyordu (Ş yanındaki İ ve , tuşunu örtüyordu)
    // Bu yüzden uzantıyı tamamen kaldırdık — Enter artık sadece 1 satırlık düz tuş.
    // Eğer L-şekil istenirse, extension tuşların *altında* çizilmeli, üstünde değil.
    for row in rows.iter() {
        let total_units: f32 = row.iter().map(|k| k.width).sum();
        let gap_total = key_gap * (row.len() as f32 - 1.0);
        let unit = (draw_width - gap_total) / total_units;

        let mut x = origin.x;
        for key in row {
            let w = key.width * unit;
            let rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, row_height));
            draw_key(ui, rect, key, active_key, dim_hand);
            x += w + key_gap;
        }

        y += row_height + row_gap;
    }
}

fn draw_key(
    ui: &mut Ui,
    rect: Rect,
    key: &Key,
    active_key: Option<char>,
    dim_hand: Option<Hand>,
) {
    let is_active = key.id.map_or(false, |c| Some(c) == active_key);

    // Boşluk tuşu her zaman görünür
    let is_space = key.id == Some(' ');

    let dimmed = match (dim_hand, key.hand) {
        (Some(dim), Some(kh)) if !is_space => dim == kh,
        _ => false,
    };

    // Aktif tuş dimlenmiş olsa bile görünür kalmalı (beyaz vurgu); boşluk asla gizlenmez
    let hidden = dimmed && !is_active && !is_space;

    let rounding = CornerRadius::same(5);

    if hidden {
        // Tuş tamamen kaybolsun: arka planla aynı (panel siyah), yazı yok, çerçeve yok
        // Görsel kaybolma için transparent yerine panel rengine yakın koyu dolgu
        // ancak tamamen görünmez yapmak için sadece çerçevesiz transparent kullanıyoruz
        // Burada hafif koyu dolgu kullanıp yazı çizmemek kaybolma hissi verir.
        // En temiz: fill TRANSPARENT + stroke TRANSPARENT + yazı yok.
        ui.painter().rect_filled(rect, rounding, Color32::TRANSPARENT);
        // Yazı çizme — kaybolsun
        return;
    }

    // Aktif tuş: kendi renginin karşıtı olarak beyaz dolgu + siyah yazı
    let fill = if is_active {
        Color32::WHITE
    } else {
        key.color
    };

    ui.painter().rect_filled(rect, rounding, fill);
    // border istenmiyor — hiç stroke çizme

    let text_color = if is_active {
        Color32::BLACK
    } else {
        Color32::BLACK
    };

    if let Some(top) = key.top {
        ui.painter().text(
            Pos2::new(rect.min.x + 4.0, rect.min.y + 3.0),
            Align2::LEFT_TOP,
            top,
            FontId::proportional(11.0),
            text_color,
        );
        ui.painter().text(
            Pos2::new(rect.max.x - 4.0, rect.max.y - 3.0),
            Align2::RIGHT_BOTTOM,
            key.bottom,
            FontId::proportional(15.0),
            text_color,
        );
    } else {
        let font_size = if key.bottom.chars().count() > 3 { 12.0 } else { 18.0 };
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            key.bottom,
            FontId::proportional(font_size),
            text_color,
        );
    }
}

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
    pub fn color(&self) -> Color32 {
        match self {
            Finger::LeftPinky => C_GREEN,
            Finger::LeftRing => C_LAVENDER,
            Finger::LeftMiddle => C_CREAM,
            Finger::LeftIndex => C_RED,
            Finger::RightIndex => C_TEAL,
            Finger::RightMiddle => C_TAN,
            Finger::RightRing => C_PINK,
            Finger::RightPinky => C_ORANGE,
            Finger::Thumb => C_INDIGO,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Finger::LeftPinky => "SOL SERÇE",
            Finger::LeftRing => "SOL YÜZÜK",
            Finger::LeftMiddle => "SOL ORTA",
            Finger::LeftIndex => "SOL İŞARET",
            Finger::RightIndex => "SAĞ İŞARET",
            Finger::RightMiddle => "SAĞ ORTA",
            Finger::RightRing => "SAĞ YÜZÜK",
            Finger::RightPinky => "SAĞ SERÇE",
            Finger::Thumb => "BAŞPARMAK",
        }
    }
}

pub fn finger_map() -> std::collections::HashMap<char, Finger> {
    let mut m = std::collections::HashMap::new();
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
    m.insert('ı', Finger::RightMiddle);
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
    m.insert(',', Finger::RightPinky);
    m.insert('.', Finger::RightPinky);
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
