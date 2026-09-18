mod app;
mod config;
mod hand_mode;
mod storage;
mod typing;
mod ui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_title("fastshi — yazım antrenörü"),
        ..Default::default()
    };
    eframe::run_native(
        "fastshi",
        options,
        Box::new(|cc| Ok(Box::new(app::TypingApp::new(cc)))),
    )
}
