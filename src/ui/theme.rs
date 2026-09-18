use eframe::egui;

pub use egui::Color32;

pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const GRAY: Color32 = Color32::from_rgb(128, 128, 128);
pub const DARK_GRAY: Color32 = Color32::from_rgb(30, 30, 30);
pub const LIGHT_GRAY: Color32 = Color32::from_rgb(200, 200, 200);

pub fn apply_dark_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(WHITE);
    visuals.panel_fill = BLACK;
    visuals.window_fill = DARK_GRAY;
    visuals.extreme_bg_color = BLACK;
    visuals.faint_bg_color = DARK_GRAY;
    visuals.widgets.noninteractive.bg_fill = DARK_GRAY;
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0_f32, WHITE);
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(20, 20, 20);
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0_f32, WHITE);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(40, 40, 40);
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0_f32, WHITE);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 60, 60);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0_f32, WHITE);
    visuals.selection.bg_fill = egui::Color32::from_rgb(80, 80, 80);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, WHITE);
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0_f32, GRAY);
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, GRAY);
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, WHITE);
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0_f32, WHITE);

    style.visuals = visuals;
    ctx.set_style(style);
}

pub fn heading_text(text: &str) -> egui::RichText {
    egui::RichText::new(text.to_uppercase())
        .family(egui::FontFamily::Name("orbitron".into()))
        .size(24.0)
        .color(WHITE)
}

pub fn sub_heading_text(text: &str) -> egui::RichText {
    egui::RichText::new(text.to_uppercase())
        .family(egui::FontFamily::Name("orbitron".into()))
        .size(18.0)
        .color(WHITE)
}

pub fn body_text(text: &str) -> egui::RichText {
    egui::RichText::new(text.to_lowercase())
        .family(egui::FontFamily::Name("spacemono".into()))
        .size(16.0)
        .color(WHITE)
}

pub fn small_text(text: &str) -> egui::RichText {
    egui::RichText::new(text.to_lowercase())
        .family(egui::FontFamily::Name("spacemono".into()))
        .size(12.0)
        .color(GRAY)
}

pub fn typing_text(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .family(egui::FontFamily::Name("spacemono".into()))
        .size(20.0)
        .color(WHITE)
}

pub fn button_text(text: &str) -> egui::RichText {
    egui::RichText::new(text.to_uppercase())
        .family(egui::FontFamily::Name("orbitron".into()))
        .size(14.0)
        .color(WHITE)
}
