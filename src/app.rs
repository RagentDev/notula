use crate::assets::AssetManager;
use crate::components::{CustomWindowFrame, TextEditor};
use eframe::egui;
use eframe::emath::Vec2;

pub struct NotepadApp {
    assets: AssetManager,
    text_editor: TextEditor,
    text_content: String
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            assets: AssetManager::new(),
            text_editor: TextEditor::new(),
            text_content: String::new()
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
            // Vertical layout for toolbar + text area
            ui.vertical(|ui| {
                // Toolbar at the top
                ui.horizontal(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {
                            self.text_content.clear();
                            ui.close_menu();
                        }
                        if ui.button("Open...").clicked() {
                            // TODO: File dialog
                            ui.close_menu();
                        }
                        if ui.button("Save").clicked() {
                            // TODO: Save file
                            ui.close_menu();
                        }
                        if ui.button("Save As...").clicked() {
                            // TODO: Save As dialog
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Select All").clicked() {
                            // TODO: Select all text
                            ui.close_menu();
                        }
                        if ui.button("Copy").clicked() {
                            // TODO: Copy to clipboard
                            ui.close_menu();
                        }
                        if ui.button("Paste").clicked() {
                            // TODO: Paste from clipboard
                            ui.close_menu();
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::widgets::global_theme_preference_buttons(ui);
                    });
                });

                ui.separator();

                let image_data = egui::ColorImage::new([64, 64], egui::Color32::from_rgb(255, 100, 100));
                let texture = ctx.load_texture("test_image", image_data, egui::TextureOptions::default());
                self.text_editor.add_image(1, texture, Vec2::new(64.0, 64.0));

                self.text_editor.show(ui, &mut self.text_content)
            });
        });
    }
}
