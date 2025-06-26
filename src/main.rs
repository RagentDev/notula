mod models;
mod editor;
mod file_ops;
mod image;
mod ui;
mod app;

use app::NotepadApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Untitled - Notepad"),
        ..Default::default()
    };

    eframe::run_native(
        "Notepad",
        options,
        Box::new(|_cc| Box::<NotepadApp>::default()),
    )
}