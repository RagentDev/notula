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

#[derive(Clone)]
enum ContentElement {
    Text(String),
    Image { id: String, width: u32, height: u32 },
}

struct NotepadApp {
    text: String,
    images: Vec<(usize, ContentElement)>, // (position_in_text, image_element)
    file_path: Option<PathBuf>,
    is_modified: bool,
    image_cache: HashMap<String, egui::TextureHandle>,
    image_data: HashMap<String, Vec<u8>>, // Store actual image data
    metadata: DocumentMetadata,
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            images: Vec::new(),
            file_path: None,
            is_modified: false,
            image_cache: HashMap::new(),
            image_data: HashMap::new(),
            metadata: DocumentMetadata {
                images: HashMap::new(),
            },
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
        let line_count = self.text.lines().count().max(1);
        let char_count = self.text.chars().count() + self.images.len(); // Include images in char count
        (line_count, char_count)
    }

    fn new_file(&mut self) {
        self.text = String::new();
        self.images.clear();
        self.file_path = None;
        self.is_modified = false;
        self.image_cache.clear();
        self.image_data.clear();
        self.metadata = DocumentMetadata {
            images: HashMap::new(),
        };
    }

    fn insert_image_from_bytes(&mut self, image_bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let img = image::load_from_memory(&image_bytes)?;
        let (width, height) = img.dimensions();
        let id = Uuid::new_v4().to_string();
        
        // Store image data and metadata
        self.image_data.insert(id.clone(), image_bytes.clone());
        let image_metadata = ImageMetadata {
            id: id.clone(),
            data: general_purpose::STANDARD.encode(&image_bytes),
            width,
            height,
        };
        self.metadata.images.insert(id.clone(), image_metadata);
        
        let image_element = ContentElement::Image {
            id: id.clone(),
            width,
            height,
        };
        
        // Insert image reference at the end of current text
        let position = self.text.len();
        self.images.push((position, image_element));
        
        self.is_modified = true;
        Ok(())
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
        let mut clipboard = arboard::Clipboard::new()?;
        let image_data = clipboard.get_image()?;
        
        let mut buffer = Vec::new();
        let img = image::RgbaImage::from_raw(
            image_data.width as u32,
            image_data.height as u32,
            image_data.bytes.into_owned(),
        ).ok_or("Failed to create image from clipboard data")?;
        
        let dynamic_img = DynamicImage::ImageRgba8(img);
        let mut cursor = std::io::Cursor::new(&mut buffer);
        dynamic_img.write_to(&mut cursor, image::ImageOutputFormat::Png)?;
        self.insert_image_from_bytes(buffer)?;
        Ok(())
    }


    fn content_to_text(&self) -> String {
        let mut result = self.text.clone();
        // Insert image placeholders at their positions
        for (_, image) in &self.images {
            if let ContentElement::Image { id, .. } = image {
                result.push_str(&format!("\n[img_load(\"{}\")]", id));
            }
        }
        result
    }

    fn text_to_content(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.images.clear();
        let mut text_content = String::new();
        
        for line in text.lines() {
            if line.starts_with("[img_load(\"") && line.ends_with("\")]") {
                // Extract image ID
                let start = "[img_load(\"".len();
                let end = line.len() - "\")]".len();
                let id = &line[start..end];
                
                if let Some(image_meta) = self.metadata.images.get(id) {
                    let image_element = ContentElement::Image {
                        id: id.to_string(),
                        width: image_meta.width,
                        height: image_meta.height,
                    };
                    
                    // Store image at current text position
                    self.images.push((text_content.len(), image_element));
                    
                    // Decode and store image data if not already present
                    if !self.image_data.contains_key(id) {
                        let image_bytes = general_purpose::STANDARD.decode(&image_meta.data)?;
                        self.image_data.insert(id.to_string(), image_bytes);
                    }
                } else {
                    // Fallback to text if image not found
                    if !text_content.is_empty() {
                        text_content.push('\n');
                    }
                    text_content.push_str(line);
                }
            } else {
                if !text_content.is_empty() {
                    text_content.push('\n');
                }
                text_content.push_str(line);
            }
        }
        
        self.text = text_content;
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
                        let _ = self.paste_image_from_clipboard();
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
            // Handle global Ctrl+V for image paste
            if ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::V)) {
                let _ = self.paste_image_from_clipboard();
            }
            
            ui.vertical(|ui| {
                // Main text editor - full screen like Windows Notepad
                let text_edit = egui::TextEdit::multiline(&mut self.text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(20);
                    
                let response = ui.add_sized(ui.available_size(), text_edit);
                
                if response.changed() {
                    self.is_modified = true;
                }
                
                // Display images below the text editor
                if !self.images.is_empty() {
                    ui.separator();
                    ui.label("Images in document:");
                    
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, true])
                        .max_height(200.0)
                        .show(ui, |ui| {
                            for (_, image_element) in &self.images {
                                if let ContentElement::Image { id, width, height } = image_element {
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
                                            0.5 // Show images at 50% size in preview
                                        };
                                        
                                        let image_size = egui::Vec2::new(
                                            *width as f32 * scale,
                                            *height as f32 * scale
                                        );
                                        
                                        ui.add(egui::Image::from_texture(texture).fit_to_exact_size(image_size));
                                    } else {
                                        ui.label("[IMAGE LOAD ERROR]");
                                    }
                                }
                            }
                        });
                }
            });
        });
    }
}