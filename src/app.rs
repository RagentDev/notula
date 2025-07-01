use crate::components::CustomWindowFrame;
use eframe::egui;
use crate::assets::AssetManager;

pub struct NotepadApp {
    assets: AssetManager,
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            assets: AssetManager::new(),
        }
    }
}

impl NotepadApp {
    pub fn get_window_title(&self) -> String {
        "Notula".to_string()
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CustomWindowFrame::show(ctx, &self.get_window_title(), &self.assets, |ui| {
            ui.label("This is just the contents of the window.");
            ui.horizontal(|ui| {
                ui.label("egui theme:");
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
    }
}
