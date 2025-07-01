mod app;
mod components;

use app::NotepadApp;
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([500.0, 500.0])
            .with_min_inner_size([300.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Notepad",
        options,
        Box::new(|_cc| Ok(Box::<NotepadApp>::default())),
    )
}