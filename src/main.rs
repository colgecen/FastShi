mod app;
mod config;
mod hand_mode;
mod sound;
mod storage;
mod typing;
mod ui;

fn main() -> eframe::Result {
    let icon_data = load_icon();

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([900.0, 600.0])
        .with_title("FastShi");

    if let Some(icon) = icon_data {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "fastshi",
        options,
        Box::new(|cc| Ok(Box::new(app::TypingApp::new(cc)))),
    )
}

fn load_icon() -> Option<egui::IconData> {
    let bytes = include_bytes!("../assets/images/FastShi_32.png");
    let img = image::load_from_memory(bytes).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    Some(egui::IconData {
        rgba: rgba.into_raw(),
        width: w,
        height: h,
    })
}
