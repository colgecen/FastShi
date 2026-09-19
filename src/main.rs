mod app;
mod config;
mod hand_mode;
mod storage;
mod typing;
mod ui;

fn main() -> eframe::Result {
    let icon_bytes = include_bytes!("../assets/images/FastShi.png");
    let icon = image::load_from_memory(icon_bytes)
        .ok()
        .and_then(|img| {
            let rgba = img.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            Some(egui::IconData {
                rgba: rgba.into_raw(),
                width: size[0] as u32,
                height: size[1] as u32,
            })
        });

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([900.0, 600.0])
        .with_title("FastShi");

    if let Some(icon_data) = icon {
        viewport = viewport.with_icon(icon_data);
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
