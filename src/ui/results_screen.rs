use eframe::egui;

use crate::typing::metrics::Metrics;
use super::theme;

pub fn show(ui: &mut egui::Ui, metrics: &Metrics, modu: &str, sure_sn: u64) {
    ui.heading(theme::heading_text("sonuçlar"));
    ui.add_space(20.0);

    ui.label(theme::sub_heading_text("test bilgisi"));
    ui.label(theme::body_text(&format!("mod: {}", modu)));
    ui.label(theme::body_text(&format!("süre: {} sn", sure_sn)));
    ui.add_space(10.0);

    ui.label(theme::sub_heading_text("performans"));
    ui.horizontal(|ui| {
        ui.label(theme::body_text(&format!("wpm: {:.1}", metrics.wpm)));
        ui.label(theme::body_text(&format!("cpm: {:.1}", metrics.cpm)));
    });
    ui.add_space(5.0);

    let dogruluk_rengi = if metrics.dogruluk_yuzde >= 90.0 {
        theme::WHITE
    } else if metrics.dogruluk_yuzde >= 70.0 {
        theme::LIGHT_GRAY
    } else {
        egui::Color32::from_rgb(200, 100, 100)
    };

    ui.label(
        egui::RichText::new(format!("doğruluk: {:.1}%", metrics.dogruluk_yuzde))
            .family(egui::FontFamily::Name("spacemono".into()))
            .size(18.0)
            .color(dogruluk_rengi),
    );
    ui.add_space(5.0);

    ui.label(theme::body_text(&format!(
        "doğru: {} | yanlış: {}",
        metrics.dogru_tus, metrics.yanlis_tus
    )));
}
