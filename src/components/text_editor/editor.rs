use super::renderer::TextEditorRenderer;

use crate::components::text_editor::util::calculate_line_height;
use eframe::epaint::StrokeKind;
use egui::WidgetType::TextEdit;
use egui::{Color32, EventFilter, FontId, Key, Pos2, Rect, Response, Stroke, Ui, Vec2};

pub type TextEditorImageMap = std::collections::HashMap<usize, (egui::TextureHandle, Vec2)>;

pub const IMAGE_PADDING: f32 = 8.0;

#[derive(Debug, Clone)]
pub struct FontMetrics {
    pub font_id: FontId,
    pub font_size: f32,
    pub char_width: f32,
    pub line_height: f32,
}

impl FontMetrics {
    pub fn new(font_size: f32) -> Self {
        Self {
            font_id: FontId::monospace(font_size),
            font_size,
            char_width: 0.0,
            line_height: 0.0,
        }
    }

    pub fn calculate(&mut self, ui: &mut Ui) {
        // self.line_number_width = ui.fonts(|f| f.glyph_width(&self.font_id, '0')) * (digits + 1) as f32;
    }
}

pub struct TextEditor {
    renderer: TextEditorRenderer,
    font_metrics: FontMetrics,
    hint_text: String,
    margin: f32,
    images: TextEditorImageMap,
    cursor_line: usize,
    cursor_column: usize,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            renderer: TextEditorRenderer::new(),
            font_metrics: FontMetrics::new(14.0),
            hint_text: "Start typing...".to_string(),
            margin: 8.0,
            images: Default::default(),
            cursor_line: 0,
            cursor_column: 0,
        }
    }

    pub fn add_image(&mut self, line_index: usize, texture: egui::TextureHandle, size: Vec2) {
        self.images.insert(line_index, (texture, size));
    }

    pub fn show(&mut self, ui: &mut Ui, text: &mut String) -> Response {
        let available_rect = ui.available_rect_before_wrap();
        let desired_size = Vec2::new(
            available_rect.width().max(100.0),
            available_rect.height().max(100.0),
        );

        let (rect, mut response) = ui.allocate_at_least(desired_size, egui::Sense::click());

        // Draw background
        ui.painter()
            .rect_filled(rect, 4.0, ui.visuals().extreme_bg_color);

        // Draw border
        let border_color = if response.has_focus() {
            ui.visuals().selection.bg_fill
        } else {
            ui.visuals().widgets.noninteractive.bg_stroke.color
        };

        ui.painter().rect_stroke(
            rect,
            4.0,
            Stroke::new(1.0, border_color),
            StrokeKind::Inside,
        );

        // Handle keyboard input
        self.handle_keyboard_input(&mut response, ui, text);

        let font_id = &self.font_metrics.font_id.clone();
        let text_rect = rect.shrink(self.margin);

        // Calculate line number width
        let line_count = if text.is_empty() {
            1
        } else {
            let base_count = text.lines().count();
            if text.ends_with('\n') {
                base_count + 1
            } else {
                base_count.max(1)
            }
        };
        let digits = line_count.to_string().len().max(2);
        let line_number_width = ui.fonts(|f| f.glyph_width(&font_id, '0')) * (digits + 1) as f32;

        let line_numbers_rect = Rect::from_min_size(
            text_rect.min,
            Vec2::new(line_number_width, text_rect.height()),
        );
        let content_rect = Rect::from_min_size(
            Pos2::new(text_rect.min.x + line_number_width, text_rect.min.y),
            Vec2::new(text_rect.width() - line_number_width, text_rect.height()),
        );

        // Handle focus & cursor clicking into text
        if response.clicked() {
            response.request_focus();
            self.handle_click_positioning(&response, ui, text, &font_id, content_rect);
        }

        // Render lines
        self.renderer.render(
            ui,
            text,
            &font_id,
            line_numbers_rect,
            content_rect,
            &self.images,
            self.cursor_line,
            self.cursor_column,
        );
        // self.render_lines(ui, text, &font_id, line_numbers_rect, content_rect);

        response
    }

    fn handle_click_positioning(
        &mut self,
        response: &Response,
        ui: &Ui,
        text: &str,
        font_id: &FontId,
        content_rect: Rect,
    ) {
        if let Some(click_pos) = response.interact_pointer_pos() {
            let relative_pos = click_pos - content_rect.min;
            let base_line_height = ui.fonts(|f| f.row_height(font_id));

            let lines: Vec<&str> = text.lines().collect();

            let max_line = if text.ends_with('\n') {
                lines.len()
            } else {
                lines.len().saturating_sub(1)
            };

            let mut current_y = 0.0;
            let mut clicked_line = None;

            // Find which line was clicked by walking through Y positions
            for (line_idx, line) in lines.iter().enumerate() {
                let line_height = calculate_line_height(line, base_line_height, &self.images);

                if relative_pos.y >= current_y && relative_pos.y < current_y + line_height {
                    clicked_line = Some(line_idx);
                    break;
                }
                current_y += line_height;
            }

            let clicked_line = clicked_line.unwrap_or(max_line);

            // Rest of character positioning logic remains the same...
            if clicked_line < lines.len() {
                if let Some(line) = lines.get(clicked_line) {
                    // Find closest character position in the line
                    let mut best_column = 0;
                    let mut best_distance = f32::INFINITY;

                    for col in 0..=line.chars().count() {
                        let text_before = line.chars().take(col).collect::<String>();
                        let x_pos = ui.fonts(|f| {
                            f.layout_no_wrap(text_before, font_id.clone(), Color32::WHITE)
                                .size()
                                .x
                        });

                        let distance = (x_pos - relative_pos.x).abs();
                        if distance < best_distance {
                            best_distance = distance;
                            best_column = col;
                        }
                    }

                    self.cursor_line = clicked_line;
                    self.cursor_column = best_column;
                }
            } else {
                // We're on the trailing newline (empty line after text ending with '\n')
                self.cursor_line = clicked_line;
                self.cursor_column = 0;
            }
        }
    }

    fn handle_keyboard_input(&mut self, response: &mut Response, ui: &mut Ui, text: &mut String) {
        if !response.has_focus() {
            return;
        }

        ui.memory_mut(|mem| {
            mem.set_focus_lock_filter(
                response.id,
                EventFilter {
                    tab: true,
                    horizontal_arrows: true,
                    vertical_arrows: true,
                    escape: false,
                },
            );
        });

        let events = ui.input(|i| i.events.clone());
        for event in events {
            match event {
                egui::Event::Text(new_text) => {
                    self.insert_text_at_cursor(text, &new_text);
                    self.cursor_column += new_text.chars().count();
                    response.mark_changed();
                }
                egui::Event::Key {
                    key, pressed: true, ..
                } => match key {
                    Key::Backspace => {
                        self.delete_char_before_cursor(text);
                        response.mark_changed();
                    }
                    Key::Enter => {
                        self.insert_text_at_cursor(text, "\n");
                        self.cursor_line += 1;
                        self.cursor_column = 0;
                        response.mark_changed();
                    }
                    Key::ArrowLeft => {
                        if self.cursor_column > 0 {
                            self.cursor_column -= 1;
                        } else if self.cursor_line > 0 {
                            // Move to end of previous line
                            self.cursor_line -= 1;
                            let lines: Vec<&str> = text.lines().collect();
                            if let Some(prev_line) = lines.get(self.cursor_line) {
                                self.cursor_column = prev_line.chars().count();
                            }
                        }
                    }
                    Key::ArrowRight => {
                        let lines: Vec<&str> = text.lines().collect();
                        if let Some(current_line) = lines.get(self.cursor_line) {
                            if self.cursor_column < current_line.chars().count() {
                                self.cursor_column += 1;
                            } else {
                                let max_line = if text.ends_with('\n') {
                                    lines.len()
                                } else {
                                    lines.len().saturating_sub(1)
                                };

                                if self.cursor_line < max_line {
                                    self.cursor_line += 1;
                                    self.cursor_column = 0;
                                }
                            }
                        }
                    }
                    Key::ArrowUp => {
                        if self.cursor_line > 0 {
                            self.cursor_line -= 1;
                            let lines: Vec<&str> = text.lines().collect();
                            if let Some(new_line) = lines.get(self.cursor_line) {
                                self.cursor_column =
                                    self.cursor_column.min(new_line.chars().count());
                            }
                        }
                    }
                    Key::ArrowDown => {
                        let lines: Vec<&str> = text.lines().collect();

                        let max_line = if text.ends_with('\n') {
                            lines.len()
                        } else {
                            lines.len().saturating_sub(1)
                        };

                        if self.cursor_line < max_line {
                            self.cursor_line += 1;
                            if let Some(new_line) = lines.get(self.cursor_line) {
                                self.cursor_column =
                                    self.cursor_column.min(new_line.chars().count());
                            } else {
                                // We're on the trailing newline, set column to 0
                                self.cursor_column = 0;
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn insert_text_at_cursor(&self, text: &mut String, new_text: &str) {
        let cursor_byte_pos = self.get_cursor_byte_position(text);
        text.insert_str(cursor_byte_pos, new_text);
    }

    fn delete_char_before_cursor(&mut self, text: &mut String) {
        if self.cursor_column > 0 {
            let cursor_byte_pos = self.get_cursor_byte_position(text);
            if let Some((char_start, _)) =
                text.char_indices().nth(cursor_byte_pos.saturating_sub(1))
            {
                text.remove(char_start);
            }
            self.cursor_column -= 1;
        }
        // ... handle line joining logic
    }

    fn get_cursor_byte_position(&self, text: &str) -> usize {
        let lines: Vec<&str> = text.lines().collect();
        let mut byte_pos = 0;

        for (i, line) in lines.iter().enumerate() {
            if i == self.cursor_line {
                byte_pos += line
                    .chars()
                    .take(self.cursor_column)
                    .map(|c| c.len_utf8())
                    .sum::<usize>();
                break;
            }
            byte_pos += line.len() + 1; // +1 for newline
        }
        byte_pos
    }
}
