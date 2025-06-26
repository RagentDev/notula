use eframe::egui;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use image::{DynamicImage, GenericImageView};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

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

#[derive(Clone, Serialize, Deserialize)]
struct ImageMetadata {
    id: String,
    data: String, // base64 encoded image data
    width: u32,
    height: u32,
}

#[derive(Clone, Serialize, Deserialize)]
struct DocumentMetadata {
    images: HashMap<String, ImageMetadata>,
}

#[derive(Clone, Debug)]
enum LineElement {
    Text(String),
    Image { id: String, width: u32, height: u32 },
}

#[derive(Clone, Debug)]
struct DocumentLine {
    elements: Vec<LineElement>,
}

impl DocumentLine {
    fn new() -> Self {
        Self {
            elements: vec![LineElement::Text(String::new())],
        }
    }
    
    fn text_content(&self) -> String {
        self.elements.iter()
            .filter_map(|e| match e {
                LineElement::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("")
    }
    
    fn is_empty(&self) -> bool {
        self.elements.iter().all(|e| match e {
            LineElement::Text(text) => text.is_empty(),
            _ => false,
        })
    }
}

struct NotepadApp {
    lines: Vec<DocumentLine>,
    cursor_line: usize,
    cursor_col: usize,
    file_path: Option<PathBuf>,
    is_modified: bool,
    image_cache: HashMap<String, egui::TextureHandle>,
    image_data: HashMap<String, Vec<u8>>,
    metadata: DocumentMetadata,
    scroll_offset: f32,
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            lines: vec![DocumentLine::new()],
            cursor_line: 0,
            cursor_col: 0,
            file_path: None,
            is_modified: false,
            image_cache: HashMap::new(),
            image_data: HashMap::new(),
            metadata: DocumentMetadata {
                images: HashMap::new(),
            },
            scroll_offset: 0.0,
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

    fn get_stats(&self) -> (usize, usize) {
        let line_count = self.lines.len();
        let char_count = self.lines.iter()
            .map(|line| line.text_content().chars().count())
            .sum::<usize>() + 
            self.lines.iter()
                .map(|line| line.elements.iter()
                    .filter(|e| matches!(e, LineElement::Image { .. }))
                    .count())
                .sum::<usize>();
        (line_count, char_count)
    }

    fn new_file(&mut self) {
        self.lines = vec![DocumentLine::new()];
        self.cursor_line = 0;
        self.cursor_col = 0;
        self.file_path = None;
        self.is_modified = false;
        self.image_cache.clear();
        self.image_data.clear();
        self.metadata = DocumentMetadata {
            images: HashMap::new(),
        };
        self.scroll_offset = 0.0;
    }

    fn insert_image_from_bytes(&mut self, image_bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Inserting image from {} bytes", image_bytes.len());
        let img = image::load_from_memory(&image_bytes)?;
        let (width, height) = img.dimensions();
        let id = Uuid::new_v4().to_string();
        
        println!("Created image ID: {}, dimensions: {}x{}", id, width, height);
        
        // Store image data and metadata
        self.image_data.insert(id.clone(), image_bytes.clone());
        let image_metadata = ImageMetadata {
            id: id.clone(),
            data: general_purpose::STANDARD.encode(&image_bytes),
            width,
            height,
        };
        self.metadata.images.insert(id.clone(), image_metadata);
        
        let image_element = LineElement::Image {
            id: id.clone(),
            width,
            height,
        };
        
        // Insert image on a new line after the current cursor position
        let new_line = DocumentLine {
            elements: vec![image_element],
        };
        
        self.cursor_line += 1;
        self.lines.insert(self.cursor_line, new_line);
        self.cursor_col = 0;
        
        // Add an empty line after the image for continued typing
        self.cursor_line += 1;
        self.lines.insert(self.cursor_line, DocumentLine::new());
        
        println!("Inserted image on line {}. Total lines: {}", self.cursor_line - 1, self.lines.len());
        
        self.is_modified = true;
        Ok(())
    }
    
    fn insert_char(&mut self, c: char) {
        if self.cursor_line >= self.lines.len() {
            self.lines.push(DocumentLine::new());
        }
        
        if let Some(LineElement::Text(text)) = self.lines[self.cursor_line].elements.first_mut() {
            text.insert(self.cursor_col, c);
            self.cursor_col += 1;
        } else {
            // If current line doesn't start with text, insert a new text element
            let mut new_text = String::new();
            new_text.insert(0, c);
            self.lines[self.cursor_line].elements.insert(0, LineElement::Text(new_text));
            self.cursor_col = 1;
        }
        self.is_modified = true;
    }
    
    fn handle_enter(&mut self) {
        if self.cursor_line >= self.lines.len() {
            self.lines.push(DocumentLine::new());
        }
        
        // Split current line at cursor position
        if let Some(LineElement::Text(text)) = self.lines[self.cursor_line].elements.first_mut() {
            let remaining_text = text.split_off(self.cursor_col);
            
            // Create new line with remaining text
            let new_line = DocumentLine {
                elements: vec![LineElement::Text(remaining_text)],
            };
            
            self.cursor_line += 1;
            self.lines.insert(self.cursor_line, new_line);
            self.cursor_col = 0;
        } else {
            // Just add a new empty line
            self.cursor_line += 1;
            self.lines.insert(self.cursor_line, DocumentLine::new());
            self.cursor_col = 0;
        }
        self.is_modified = true;
    }
    
    fn handle_backspace(&mut self) {
        if self.cursor_col > 0 {
            // Delete character in current line
            if let Some(LineElement::Text(text)) = self.lines[self.cursor_line].elements.first_mut() {
                if self.cursor_col <= text.len() {
                    text.remove(self.cursor_col - 1);
                    self.cursor_col -= 1;
                    self.is_modified = true;
                }
            }
        } else if self.cursor_line > 0 {
            // Merge with previous line
            let current_line = self.lines.remove(self.cursor_line);
            self.cursor_line -= 1;
            
            if let Some(LineElement::Text(prev_text)) = self.lines[self.cursor_line].elements.first_mut() {
                self.cursor_col = prev_text.len();
                if let Some(LineElement::Text(current_text)) = current_line.elements.first() {
                    prev_text.push_str(current_text);
                }
            }
            self.is_modified = true;
        }
    }
    
    fn move_cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = self.lines[self.cursor_line].text_content().len();
        }
    }
    
    fn move_cursor_right(&mut self) {
        let current_line_len = self.lines.get(self.cursor_line)
            .map(|line| line.text_content().len())
            .unwrap_or(0);
            
        if self.cursor_col < current_line_len {
            self.cursor_col += 1;
        } else if self.cursor_line < self.lines.len() - 1 {
            self.cursor_line += 1;
            self.cursor_col = 0;
        }
    }
    
    fn move_cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            let line_len = self.lines[self.cursor_line].text_content().len();
            self.cursor_col = self.cursor_col.min(line_len);
        }
    }
    
    fn move_cursor_down(&mut self) {
        if self.cursor_line < self.lines.len() - 1 {
            self.cursor_line += 1;
            let line_len = self.lines[self.cursor_line].text_content().len();
            self.cursor_col = self.cursor_col.min(line_len);
        }
    }
    
    fn handle_input(&mut self, ui: &mut egui::Ui) {
        // Handle text input
        ui.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Text(text) => {
                        for c in text.chars() {
                            if c.is_control() {
                                continue;
                            }
                            self.insert_char(c);
                        }
                    }
                    egui::Event::Key { key, pressed: true, modifiers: _, .. } => {
                        match key {
                            egui::Key::Enter => self.handle_enter(),
                            egui::Key::Backspace => self.handle_backspace(),
                            egui::Key::ArrowLeft => self.move_cursor_left(),
                            egui::Key::ArrowRight => self.move_cursor_right(),
                            egui::Key::ArrowUp => self.move_cursor_up(),
                            egui::Key::ArrowDown => self.move_cursor_down(),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        });
        
        // Keep cursor in view
        let line_height = 14.0;
        let cursor_y = self.cursor_line as f32 * line_height;
        
        // Basic auto-scroll
        if cursor_y < self.scroll_offset {
            self.scroll_offset = cursor_y;
        } else if cursor_y > self.scroll_offset + ui.available_height() - line_height {
            self.scroll_offset = cursor_y - ui.available_height() + line_height * 2.0;
        }
    }

    fn insert_sample_image(&mut self) {
        // Create a simple colored rectangle as a sample image
        let width = 200u32;
        let height = 100u32;
        let mut img_buffer = image::RgbaImage::new(width, height);
        
        // Fill with a gradient
        for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = 128u8;
            *pixel = image::Rgba([r, g, b, 255]);
        }
        
        let dynamic_img = DynamicImage::ImageRgba8(img_buffer);
        let mut buffer = Vec::new();
        let _ = dynamic_img.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png);
        let _ = self.insert_image_from_bytes(buffer);
    }

    fn paste_image_from_clipboard(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Attempting to paste image from clipboard...");
        let mut clipboard = arboard::Clipboard::new()?;
        let image_data = clipboard.get_image()?;
        
        println!("Got image data: {}x{}", image_data.width, image_data.height);
        
        let mut buffer = Vec::new();
        let img = image::RgbaImage::from_raw(
            image_data.width as u32,
            image_data.height as u32,
            image_data.bytes.into_owned(),
        ).ok_or("Failed to create image from clipboard data")?;
        
        let dynamic_img = DynamicImage::ImageRgba8(img);
        let mut cursor = std::io::Cursor::new(&mut buffer);
        dynamic_img.write_to(&mut cursor, image::ImageOutputFormat::Png)?;
        
        println!("Created PNG buffer of {} bytes", buffer.len());
        self.insert_image_from_bytes(buffer)?;
        println!("Successfully inserted image. Total lines: {}", self.lines.len());
        Ok(())
    }


    fn content_to_text(&self) -> String {
        self.lines.iter()
            .map(|line| {
                line.elements.iter()
                    .map(|element| match element {
                        LineElement::Text(text) => text.clone(),
                        LineElement::Image { id, .. } => format!("[img_load(\"{}\")]", id),
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn text_to_content(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.lines.clear();
        
        for line_text in text.lines() {
            if line_text.starts_with("[img_load(\"") && line_text.ends_with("\")]") {
                // Extract image ID
                let start = "[img_load(\"".len();
                let end = line_text.len() - "\")]".len();
                let id = &line_text[start..end];
                
                if let Some(image_meta) = self.metadata.images.get(id) {
                    let image_element = LineElement::Image {
                        id: id.to_string(),
                        width: image_meta.width,
                        height: image_meta.height,
                    };
                    
                    let line = DocumentLine {
                        elements: vec![image_element],
                    };
                    self.lines.push(line);
                    
                    // Decode and store image data if not already present
                    if !self.image_data.contains_key(id) {
                        let image_bytes = general_purpose::STANDARD.decode(&image_meta.data)?;
                        self.image_data.insert(id.to_string(), image_bytes);
                    }
                } else {
                    // Fallback to text line
                    let line = DocumentLine {
                        elements: vec![LineElement::Text(line_text.to_string())],
                    };
                    self.lines.push(line);
                }
            } else {
                let line = DocumentLine {
                    elements: vec![LineElement::Text(line_text.to_string())],
                };
                self.lines.push(line);
            }
        }
        
        if self.lines.is_empty() {
            self.lines.push(DocumentLine::new());
        }
        
        self.cursor_line = 0;
        self.cursor_col = 0;
        Ok(())
    }

    fn save_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.file_path {
            self.save_to_path(path.clone())?;
        }
        Ok(())
    }

    fn save_file_as(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.save_to_path(path.clone())?;
        self.file_path = Some(path);
        Ok(())
    }

    fn save_to_path(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let text_content = self.content_to_text();
        
        // Determine file extension
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("txt");
        
        match extension {
            "md" | "txt" => {
                // Save as plain text with image placeholders
                fs::write(&path, text_content)?;
                
                // Save metadata file if there are images
                if !self.metadata.images.is_empty() {
                    let metadata_path = path.with_extension(format!("{}.meta", extension));
                    let metadata_json = serde_json::to_string_pretty(&self.metadata)?;
                    fs::write(metadata_path, metadata_json)?;
                }
            }
            _ => {
                // Default to txt format
                fs::write(&path, text_content)?;
                if !self.metadata.images.is_empty() {
                    let metadata_path = path.with_extension("txt.meta");
                    let metadata_json = serde_json::to_string_pretty(&self.metadata)?;
                    fs::write(metadata_path, metadata_json)?;
                }
            }
        }
        
        self.is_modified = false;
        Ok(())
    }

    fn open_file(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Read the text content
        let text_content = fs::read_to_string(&path)?;
        
        // Try to load metadata file
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("txt");
        let metadata_path = path.with_extension(format!("{}.meta", extension));
        
        if metadata_path.exists() {
            let metadata_json = fs::read_to_string(metadata_path)?;
            self.metadata = serde_json::from_str(&metadata_json)?;
        } else {
            self.metadata = DocumentMetadata {
                images: HashMap::new(),
            };
        }
        
        // Clear existing data
        self.image_cache.clear();
        self.image_data.clear();
        
        // Parse content
        self.text_to_content(&text_content)?;
        
        self.file_path = Some(path);
        self.is_modified = false;
        Ok(())
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                        // TODO: Implement text paste
                        ui.close_menu();
                    }
                    if ui.button("Paste Image (Ctrl+V)").clicked() {
                        if let Err(e) = self.paste_image_from_clipboard() {
                            println!("Error pasting image: {}", e);
                        }
                        ui.close_menu();
                    }
                    if ui.button("Insert Sample Image").clicked() {
                        self.insert_sample_image();
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
                let (line_count, char_count) = self.get_stats();
                ui.label(format!("Lines: {}", line_count));
                ui.separator();
                ui.label(format!("Characters: {}", char_count));
            });
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Handle input
            self.handle_input(ui);
            
            // Handle global Ctrl+V for image paste
            if ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::V)) {
                if let Err(e) = self.paste_image_from_clipboard() {
                    println!("Error pasting image: {}", e);
                }
            }
            
            // Custom editor with line numbers
            ui.horizontal(|ui| {
                // Line numbers column
                let line_height = 14.0;
                
                ui.vertical(|ui| {
                    ui.set_width(50.0);
                    ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_gray(240);
                    
                    // Show line numbers for all lines
                    for line_idx in 0..self.lines.len() {
                        let line_num = line_idx + 1;
                        ui.horizontal(|ui| {
                            ui.set_min_height(line_height);
                            let text = format!("{:4}", line_num);
                            let color = if line_idx == self.cursor_line {
                                egui::Color32::from_rgb(0, 100, 200)
                            } else {
                                egui::Color32::from_gray(120)
                            };
                            ui.colored_label(color, text);
                        });
                    }
                });
                
                ui.separator();
                
                // Main editor area - use all available space
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            for (line_idx, line) in self.lines.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.set_min_height(line_height);
                                    
                                    // Render line content
                                    for element in &line.elements {
                                        match element {
                                            LineElement::Text(text) => {
                                                // Show cursor if this is the current line
                                                if line_idx == self.cursor_line {
                                                    // Create editable text with cursor
                                                    let display_text = if text.is_empty() && line.elements.len() == 1 {
                                                        " ".to_string() // Show space for empty lines
                                                    } else {
                                                        text.clone()
                                                    };
                                                    
                                                    // Calculate cursor position
                                                    let before_cursor = display_text.chars().take(self.cursor_col).collect::<String>();
                                                    let cursor_char = display_text.chars().nth(self.cursor_col).unwrap_or(' ');
                                                    let after_cursor = display_text.chars().skip(self.cursor_col + 1).collect::<String>();
                                                    
                                                    ui.horizontal(|ui| {
                                                        ui.label(&before_cursor);
                                                        ui.colored_label(egui::Color32::BLACK, format!("|{}", cursor_char));
                                                        ui.label(&after_cursor);
                                                    });
                                                } else {
                                                    // Regular text display
                                                    ui.label(text);
                                                }
                                            }
                                            LineElement::Image { id, width, height } => {
                                                // Load image into texture cache if not already loaded
                                                if !self.image_cache.contains_key(id) {
                                                    if let Some(image_bytes) = self.image_data.get(id) {
                                                        if let Ok(image) = image::load_from_memory(image_bytes) {
                                                            let size = [image.width() as usize, image.height() as usize];
                                                            let image_buffer = image.to_rgba8();
                                                            let pixels = image_buffer.as_flat_samples();
                                                            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                                                size,
                                                                pixels.as_slice(),
                                                            );
                                                            let texture = ctx.load_texture(id.clone(), color_image, egui::TextureOptions::default());
                                                            self.image_cache.insert(id.clone(), texture);
                                                        }
                                                    }
                                                }
                                                
                                                if let Some(texture) = self.image_cache.get(id) {
                                                    let max_width = ui.available_width() - 20.0;
                                                    let scale = if *width as f32 > max_width {
                                                        max_width / *width as f32
                                                    } else {
                                                        1.0
                                                    };
                                                    
                                                    let image_size = egui::Vec2::new(
                                                        *width as f32 * scale,
                                                        *height as f32 * scale
                                                    );
                                                    
                                                    let response = ui.add(egui::Image::from_texture(texture).fit_to_exact_size(image_size));
                                                    
                                                    // Allow clicking on image to position cursor
                                                    if response.clicked() {
                                                        self.cursor_line = line_idx;
                                                        self.cursor_col = 0;
                                                    }
                                                } else {
                                                    ui.label("[IMAGE LOAD ERROR]");
                                                }
                                            }
                                        }
                                    }
                                });
                            }
                        });
                    });
            });
        });
    }
}