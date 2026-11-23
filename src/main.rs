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
        .with_inner_size([1440.0, 900.0])
        .with_maximized(true)
        .with_app_id("fastshi")
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

const ICON_SIZE: u32 = 256;
const ICON_CORNER_RADIUS: f32 = 22.0;

fn load_icon() -> Option<egui::IconData> {
    let bytes = include_bytes!("../assets/images/FastShi.png");
    let img = image::load_from_memory(bytes).ok()?;
    let resized = image::imageops::resize(
        &img.to_rgba8(),
        ICON_SIZE,
        ICON_SIZE,
        image::imageops::FilterType::Lanczos3,
    );
    let mut pixels = resized.into_raw();
    apply_rounded_corners(&mut pixels, ICON_SIZE, ICON_SIZE, ICON_CORNER_RADIUS);
    Some(egui::IconData {
        rgba: pixels,
        width: ICON_SIZE,
        height: ICON_SIZE,
    })
}

fn apply_rounded_corners(pixels: &mut [u8], width: u32, height: u32, radius: f32) {
    if width == 0 || height == 0 || radius <= 0.0 {
        return;
    }

    let w = width as usize;
    let h = height as usize;
    let r = radius
        .min(width as f32 / 2.0)
        .min(height as f32 / 2.0);

    for y in 0..h {
        for x in 0..w {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let cx = px.clamp(r, w as f32 - r);
            let cy = py.clamp(r, h as f32 - r);
            let dist = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
            let coverage = (r - dist + 0.5).clamp(0.0, 1.0);

            if coverage < 1.0 {
                let alpha_idx = (y * w + x) * 4 + 3;
                pixels[alpha_idx] =
                    (pixels[alpha_idx] as f32 * coverage).round() as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ikon_koseleri_yuvarlatilir() {
        let mut pixels = vec![255u8; 10 * 10 * 4];
        apply_rounded_corners(&mut pixels, 10, 10, 3.0);

        assert_eq!(pixels[3], 0, "sol üst köşe tamamen saydam olmalı");
        assert_eq!(pixels[(5 * 10 + 5) * 4 + 3], 255);
        assert_eq!(pixels[(5 * 10 + 0) * 4 + 3], 255);
    }
}
