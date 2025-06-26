use crate::models::{DocumentLine, DocumentMetadata};
use crate::editor::{EditorCore, EditorRenderer};
use crate::file_ops::{save_to_path, load_from_path};
use crate::image::{create_sample_image, get_image_from_clipboard, process_image_bytes};
use crate::ui::{MenuBar, StatusBar};
use eframe::egui;
use std::path::PathBuf;
use std::collections::HashMap;

pub struct NotepadApp {
    pub lines: Vec<DocumentLine>,
    pub file_path: Option<PathBuf>,
    pub is_modified: bool,
    pub image_cache: HashMap<String, egui::TextureHandle>,
    pub image_data: HashMap<String, Vec<u8>>,
    pub metadata: DocumentMetadata,
    pub editor: EditorCore,
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            lines: vec![DocumentLine::new()],
            file_path: None,
            is_modified: false,
            image_cache: HashMap::new(),
            image_data: HashMap::new(),
            metadata: DocumentMetadata {
                images: HashMap::new(),
            },
            editor: EditorCore::default(),
        }
    }
}

impl NotepadApp {
    pub fn get_window_title(&self) -> String {
        let filename = self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled");
        
        let modified_marker = if self.is_modified { "*" } else { "" };
        format!("{}{} - Notepad", modified_marker, filename)
    }

    pub fn new_file(&mut self) {
        self.lines = vec![DocumentLine::new()];
        self.file_path = None;
        self.is_modified = false;
        self.image_cache.clear();
        self.image_data.clear();
        self.metadata = DocumentMetadata {
            images: HashMap::new(),
        };
        self.editor.reset();
    }

    pub fn insert_image_from_bytes(&mut self, image_bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        process_image_bytes(
            image_bytes,
            &mut self.lines,
            &mut self.editor.cursor_line,
            &mut self.editor.cursor_col,
            &mut self.image_data,
            &mut self.metadata.images,
        )?;
        self.is_modified = true;
        Ok(())
    }

    pub fn insert_sample_image(&mut self) {
        if let Ok(buffer) = create_sample_image() {
            let _ = self.insert_image_from_bytes(buffer);
        }
    }

    pub fn paste_image_from_clipboard(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let buffer = get_image_from_clipboard()?;
        self.insert_image_from_bytes(buffer)?;
        println!("Successfully inserted image. Total lines: {}", self.lines.len());
        Ok(())
    }

    pub fn save_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.file_path {
            save_to_path(path.clone(), &self.lines, &self.metadata)?;
            self.is_modified = false;
        }
        Ok(())
    }

    pub fn save_file_as(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        save_to_path(path.clone(), &self.lines, &self.metadata)?;
        self.file_path = Some(path);
        self.is_modified = false;
        Ok(())
    }

    pub fn open_file(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let (lines, metadata, image_data) = load_from_path(path.clone())?;
        
        // Clear existing data
        self.image_cache.clear();
        
        // Update app state
        self.lines = lines;
        self.metadata = metadata;
        self.image_data = image_data;
        self.file_path = Some(path);
        self.is_modified = false;
        self.editor.reset();
        
        Ok(())
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update window title
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.get_window_title()));

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            let actions = MenuBar::render(ui, ctx);
            
            if actions.new_file {
                self.new_file();
            }
            if actions.open_file {
                // TODO: Implement file dialog
            }
            if actions.save_file {
                if self.file_path.is_some() {
                    let _ = self.save_file();
                }
            }
            if actions.save_as {
                // TODO: Implement file dialog
            }
            if actions.paste_image {
                if let Err(e) = self.paste_image_from_clipboard() {
                    println!("Error pasting image: {}", e);
                }
            }
            if actions.insert_sample_image {
                self.insert_sample_image();
            }
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            StatusBar::render(ui, &self.lines);
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Handle input and get modification status
            let modified = self.editor.handle_input(ui, &mut self.lines);
            if modified {
                self.is_modified = true;
            }
            
            // Handle global Ctrl+V for image paste
            if ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::V)) {
                if let Err(e) = self.paste_image_from_clipboard() {
                    println!("Error pasting image: {}", e);
                }
            }
            
            // Render the editor
            EditorRenderer::render(
                ui,
                ctx,
                &mut self.editor,
                &self.lines,
                &mut self.image_cache,
                &self.image_data,
            );
        });
    }
}