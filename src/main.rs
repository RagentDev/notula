use eframe::egui;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you want logging)

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

struct NotepadApp {
    text: String,
    file_path: Option<PathBuf>,
    is_modified: bool,
    cursor_pos: usize,
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            file_path: None,
            is_modified: false,
            cursor_pos: 0,
        }
    }
}

impl NotepadApp {
    fn get_window_title(&self) -> String {
        let filename = self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled");
        
        let modified_marker = if self.is_modified { "*" } else { "" };
        format!("{}{} - Notepad", modified_marker, filename)
    }

    fn get_line_col_from_cursor(&self) -> (usize, usize) {
        let text_before_cursor = &self.text[..self.cursor_pos.min(self.text.len())];
        let line = text_before_cursor.lines().count();
        let col = text_before_cursor.lines().last().map_or(1, |line| line.len() + 1);
        (line, col)
    }

    fn new_file(&mut self) {
        self.text.clear();
        self.file_path = None;
        self.is_modified = false;
        self.cursor_pos = 0;
    }

    fn save_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.file_path {
            fs::write(path, &self.text)?;
            self.is_modified = false;
        }
        Ok(())
    }

    fn save_file_as(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(&path, &self.text)?;
        self.file_path = Some(path);
        self.is_modified = false;
        Ok(())
    }

    fn open_file(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&path)?;
        self.text = content;
        self.file_path = Some(path);
        self.is_modified = false;
        self.cursor_pos = 0;
        Ok(())
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Update window title
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.get_window_title()));

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.new_file();
                        ui.close_menu();
                    }
                    if ui.button("Open...").clicked() {
                        // TODO: Implement file dialog
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Save").clicked() {
                        if self.file_path.is_some() {
                            let _ = self.save_file();
                        }
                        ui.close_menu();
                    }
                    if ui.button("Save As...").clicked() {
                        // TODO: Implement file dialog
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        // TODO: Implement undo
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Cut").clicked() {
                        // TODO: Implement cut
                        ui.close_menu();
                    }
                    if ui.button("Copy").clicked() {
                        // TODO: Implement copy
                        ui.close_menu();
                    }
                    if ui.button("Paste").clicked() {
                        // TODO: Implement paste
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Select All").clicked() {
                        // TODO: Implement select all
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Find...").clicked() {
                        // TODO: Implement find
                        ui.close_menu();
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Word Wrap").clicked() {
                        // TODO: Implement word wrap toggle
                        ui.close_menu();
                    }
                    if ui.button("Font...").clicked() {
                        // TODO: Implement font selection
                        ui.close_menu();
                    }
                });
            });
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let (line, col) = self.get_line_col_from_cursor();
                let char_count = self.text.chars().count();
                ui.label(format!("Ln {}, Col {}", line, col));
                ui.separator();
                ui.label(format!("{} characters", char_count));
            });
        });

        // Main text editor
        egui::CentralPanel::default().show(ctx, |ui| {
            let text_edit = egui::TextEdit::multiline(&mut self.text)
                .desired_width(f32::INFINITY)
                .desired_rows(0);
            
            let response = ui.add_sized(ui.available_size(), text_edit);
            
            // Track text changes
            if response.changed() {
                self.is_modified = true;
            }
            
            // Update cursor position if text editor has focus
            if response.has_focus() {
                // For now, just use text length as cursor position approximation
                // This is a simplified approach since egui 0.24 doesn't expose cursor position easily
                self.cursor_pos = self.text.len();
            }
        });
    }
}