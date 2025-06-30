use crate::editor::EditorCore;
use crate::models::{DocumentLine, LineElement};
use eframe::egui;
use std::collections::HashMap;

pub struct EditorRenderer;

impl EditorRenderer {
    fn render_text_with_selection_and_cursor(
        ui: &mut egui::Ui,
        text: &str,
        line_idx: usize,
        editor: &EditorCore,
    ) {
        let chars: Vec<char> = text.chars().collect();
        let line_len = chars.len();

        // Check if this line has selection
        let (selection_start_col, selection_end_col) = if editor.has_selection() {
            let (start_line, start_col) = editor.selection_start.unwrap();
            let (end_line, end_col) = editor.selection_end.unwrap();

            let (start_line, start_col, end_line, end_col) =
                if start_line < end_line || (start_line == end_line && start_col < end_col) {
                    (start_line, start_col, end_line, end_col)
                } else {
                    (end_line, end_col, start_line, start_col)
                };

            if line_idx >= start_line && line_idx <= end_line {
                let sel_start = if line_idx == start_line { start_col } else { 0 };
                let sel_end = if line_idx == end_line { end_col } else { line_len };
                (Some(sel_start), Some(sel_end))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        let show_cursor = line_idx == editor.cursor_line && editor.cursor_visible;
        let cursor_col = editor.cursor_col;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // Split text into segments around cursor and selection
            let mut current_pos = 0;

            while current_pos <= line_len {
                // Show cursor if it's at this position
                if show_cursor && cursor_col == current_pos {
                    ui.colored_label(egui::Color32::WHITE, "|");
                }

                if current_pos < line_len {
                    // Find the end of the current segment (next cursor position or selection boundary)
                    let mut segment_end = current_pos + 1;

                    // Check if we're in a selection
                    let in_selection = if let (Some(sel_start), Some(sel_end)) = (selection_start_col, selection_end_col) {
                        current_pos >= sel_start && current_pos < sel_end
                    } else {
                        false
                    };

                    // Extend segment while selection state remains the same
                    while segment_end < line_len {
                        let next_in_selection = if let (Some(sel_start), Some(sel_end)) = (selection_start_col, selection_end_col) {
                            segment_end >= sel_start && segment_end < sel_end
                        } else {
                            false
                        };

                        if next_in_selection != in_selection || (show_cursor && cursor_col == segment_end) {
                            break;
                        }
                        segment_end += 1;
                    }

                    // Render the segment
                    let segment: String = chars[current_pos..segment_end].iter().collect();

                    if in_selection {
                        let mut job = egui::text::LayoutJob::default();
                        job.append(
                            &segment,
                            0.0,
                            egui::TextFormat {
                                background: egui::Color32::from_rgb(173, 214, 255),
                                color: egui::Color32::BLACK,
                                ..Default::default()
                            },
                        );
                        ui.label(job);
                    } else {
                        ui.label(&segment);
                    }

                    current_pos = segment_end;
                } else {
                    break;
                }
            }
        });
    }

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
            // Calculate line heights first
            let base_line_height = 14.0;
            let mut line_heights: Vec<f32> = Vec::new();

            for line in lines {
                let mut max_height: f32 = base_line_height;

                for element in &line.elements {
                    if let LineElement::Image { width, height, .. } = element {
                        let max_width = ui.available_width() - 70.0; // Account for line numbers + padding
                        let scale = if *width as f32 > max_width {
                            max_width / *width as f32
                        } else {
                            1.0
                        };
                        let scaled_height = *height as f32 * scale;
                        max_height = max_height.max(scaled_height);
                    }
                }
                line_heights.push(max_height);
            }

            // Line numbers column
            ui.vertical(|ui| {
                ui.set_width(50.0);
                ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_gray(240);

                // Show line numbers with calculated heights
                for (line_idx, &line_height) in line_heights.iter().enumerate() {
                    let line_num = line_idx + 1;
                    ui.horizontal(|ui| {
                        ui.set_min_height(line_height);
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(50.0, line_height),
                            egui::Layout::top_down(egui::Align::RIGHT),
                            |ui| {
                                let text = format!("{:4}", line_num);
                                let color = if line_idx == editor.cursor_line {
                                    egui::Color32::from_rgb(0, 100, 200)
                                } else {
                                    egui::Color32::from_gray(120)
                                };
                                ui.colored_label(color, text);
                            },
                        );
                    });
                }
            });

            ui.separator();

            // Main editor area
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for (line_idx, line) in lines.iter().enumerate() {
                            let line_height = line_heights[line_idx];

                            ui.horizontal(|ui| {
                                ui.set_min_height(line_height);

                                // Render line content
                                for element in &line.elements {
                                    match element {
                                        LineElement::Text(text) => {
                                            Self::render_text_with_selection_and_cursor(
                                                ui, text, line_idx, editor
                                            );
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
