use eframe::egui;

use crate::storage::db::Database;
use super::theme;

pub fn show(ui: &mut egui::Ui) {
    ui.heading(theme::heading_text("geçmiş"));
    ui.add_space(10.0);

    let db = Database::open();
    let results = db.getir_tum();

    if results.is_empty() {
        ui.label(theme::body_text("henüz test sonucu yok"));
        return;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("stats_grid").striped(true).show(ui, |ui| {
            ui.label(theme::small_text("tarih"));
            ui.label(theme::small_text("mod"));
            ui.label(theme::small_text("wpm"));
            ui.label(theme::small_text("cpm"));
            ui.label(theme::small_text("doğruluk"));
            ui.label(theme::small_text("zorluk"));
            ui.end_row();

            for r in &results {
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

    ui.add_space(10.0);

    if !results.is_empty() {
        let avg_wpm: f64 = results.iter().map(|r| r.wpm).sum::<f64>() / results.len() as f64;
        let avg_cpm: f64 = results.iter().map(|r| r.cpm).sum::<f64>() / results.len() as f64;
        let avg_dog: f64 = results.iter().map(|r| r.dogruluk_yuzde).sum::<f64>() / results.len() as f64;

        ui.label(theme::sub_heading_text("ortalamalar"));
        ui.label(theme::body_text(&format!(
            "wpm: {:.1} | cpm: {:.1} | doğruluk: {:.1}%",
            avg_wpm, avg_cpm, avg_dog
        )));
    }
}
