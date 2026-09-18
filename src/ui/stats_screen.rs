use eframe::egui;

use crate::storage::db::Database;
use super::theme;

pub fn show(ui: &mut egui::Ui) {
    ui.heading(theme::heading_text("gecmis"));
    ui.add_space(10.0);

    let db = Database::open();
    let results = db.getir_tum();

    if results.is_empty() {
        ui.label(theme::body_text("henuz test sonucu yok"));
        return;
    }

    // El bazli gruplama
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

    egui::ScrollArea::vertical().show(ui, |ui| {
        // Iki el
        ui.label(theme::sub_heading_text("iki el"));
        ui.add_space(4.0);
        if iki_el.is_empty() {
            ui.label(theme::small_text("sonuc yok"));
        } else {
            show_group_stats(ui, &iki_el);
        }

        ui.add_space(12.0);

        // Sag el
        ui.label(theme::sub_heading_text("sag el"));
        ui.add_space(4.0);
        if sag_el.is_empty() {
            ui.label(theme::small_text("sonuc yok"));
        } else {
            show_group_stats(ui, &sag_el);
        }

        ui.add_space(12.0);

        // Sol el
        ui.label(theme::sub_heading_text("sol el"));
        ui.add_space(4.0);
        if sol_el.is_empty() {
            ui.label(theme::small_text("sonuc yok"));
        } else {
            show_group_stats(ui, &sol_el);
        }

        ui.add_space(16.0);

        // Tum sonuclar tablosu
        ui.label(theme::sub_heading_text("tum sonuclar"));
        ui.add_space(4.0);

        egui::Grid::new("stats_grid")
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
}

fn show_group_stats(ui: &mut egui::Ui, results: &[&crate::storage::models::TestResult]) {
    let count = results.len();
    let avg_wpm: f64 = results.iter().map(|r| r.wpm).sum::<f64>() / count as f64;
    let avg_cpm: f64 = results.iter().map(|r| r.cpm).sum::<f64>() / count as f64;
    let avg_dog: f64 = results.iter().map(|r| r.dogruluk_yuzde).sum::<f64>() / count as f64;
    let best_wpm = results.iter().map(|r| r.wpm).fold(0.0_f64, f64::max);
    let total_sn: u64 = results.iter().map(|r| r.sure_sn).sum();

    let frame = egui::Frame::NONE
        .fill(theme::DARK_GRAY)
        .corner_radius(egui::CornerRadius::same(6))
        .inner_margin(egui::Margin::same(10))
        .stroke(egui::Stroke::new(1.0_f32, theme::GRAY));

    frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());

        egui::Grid::new(format!("group_{}", results.len()))
            .num_columns(2)
            .spacing([20.0, 4.0])
            .show(ui, |ui| {
                ui.label(theme::small_text("test sayisi"));
                ui.label(theme::body_text(&format!("{}", count)));
                ui.end_row();

                ui.label(theme::small_text("ort. wpm"));
                ui.label(
                    egui::RichText::new(format!("{:.1}", avg_wpm))
                        .family(egui::FontFamily::Name("spacemono".into()))
                        .size(16.0)
                        .color(theme::WHITE),
                );
                ui.end_row();

                ui.label(theme::small_text("en iyi wpm"));
                ui.label(
                    egui::RichText::new(format!("{:.1}", best_wpm))
                        .family(egui::FontFamily::Name("spacemono".into()))
                        .size(16.0)
                        .color(theme::WHITE),
                );
                ui.end_row();

                ui.label(theme::small_text("ort. cpm"));
                ui.label(theme::body_text(&format!("{:.1}", avg_cpm)));
                ui.end_row();

                ui.label(theme::small_text("ort. dogruluk"));
                let dogruluk_rengi = if avg_dog >= 90.0 {
                    theme::WHITE
                } else if avg_dog >= 70.0 {
                    theme::LIGHT_GRAY
                } else {
                    egui::Color32::from_rgb(200, 100, 100)
                };
                ui.label(
                    egui::RichText::new(format!("{:.1}%", avg_dog))
                        .family(egui::FontFamily::Name("spacemono".into()))
                        .size(16.0)
                        .color(dogruluk_rengi),
                );
                ui.end_row();

                ui.label(theme::small_text("toplam sure"));
                let dakika = total_sn as f64 / 60.0;
                ui.label(theme::body_text(&format!("{:.1} dk", dakika)));
                ui.end_row();
            });
    });
}
