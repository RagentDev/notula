use crate::models::{DocumentLine, LineElement};
use crate::editor::EditorCore;
use eframe::egui;
use std::collections::HashMap;

pub struct EditorRenderer;

impl EditorRenderer {
    pub fn render(
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        editor: &mut EditorCore,
        lines: &[DocumentLine],
        image_cache: &mut HashMap<String, egui::TextureHandle>,
        image_data: &HashMap<String, Vec<u8>>,
    ) {
        // Custom editor with line numbers
        ui.horizontal(|ui| {
            // Line numbers column
            let line_height = 14.0;
            
            ui.vertical(|ui| {
                ui.set_width(50.0);
                ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_gray(240);
                
                // Show line numbers for all lines
                for line_idx in 0..lines.len() {
                    let line_num = line_idx + 1;
                    ui.horizontal(|ui| {
                        ui.set_min_height(line_height);
                        let text = format!("{:4}", line_num);
                        let color = if line_idx == editor.cursor_line {
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
                        for (line_idx, line) in lines.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.set_min_height(line_height);
                                
                                // Render line content
                                for element in &line.elements {
                                    match element {
                                        LineElement::Text(text) => {
                                            // Show cursor if this is the current line
                                            if line_idx == editor.cursor_line {
                                                // Create editable text with cursor
                                                let display_text = if text.is_empty() && line.elements.len() == 1 {
                                                    " ".to_string() // Show space for empty lines
                                                } else {
                                                    text.clone()
                                                };
                                                
                                                // Calculate cursor position
                                                let before_cursor = display_text.chars().take(editor.cursor_col).collect::<String>();
                                                let cursor_char = display_text.chars().nth(editor.cursor_col).unwrap_or(' ');
                                                let after_cursor = display_text.chars().skip(editor.cursor_col + 1).collect::<String>();
                                                
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
                                            if !image_cache.contains_key(id) {
                                                if let Some(image_bytes) = image_data.get(id) {
                                                    if let Ok(image) = image::load_from_memory(image_bytes) {
                                                        let size = [image.width() as usize, image.height() as usize];
                                                        let image_buffer = image.to_rgba8();
                                                        let pixels = image_buffer.as_flat_samples();
                                                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                                            size,
                                                            pixels.as_slice(),
                                                        );
                                                        let texture = ctx.load_texture(id.clone(), color_image, egui::TextureOptions::default());
                                                        image_cache.insert(id.clone(), texture);
                                                    }
                                                }
                                            }
                                            
                                            if let Some(texture) = image_cache.get(id) {
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
                                                    editor.cursor_line = line_idx;
                                                    editor.cursor_col = 0;
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
    }
}