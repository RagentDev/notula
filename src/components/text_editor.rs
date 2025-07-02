use eframe::epaint::StrokeKind;
use egui::{Color32, FontId, Key, Pos2, Rect, Response, Stroke, Ui, Vec2};

pub struct TextEditor {
    font_size: f32,
    hint_text: String,
    margin: f32,
    images: std::collections::HashMap<usize, (egui::TextureHandle, Vec2)>,
    cursor_line: usize,
    cursor_column: usize,
}

impl Default for TextEditor {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            hint_text: "Start typing...".to_string(),
            margin: 8.0,
            images: Default::default(),
            cursor_line: 0,
            cursor_column: 0
        }
    }
}

impl TextEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn hint_text<S: Into<String>>(mut self, hint: S) -> Self {
        self.hint_text = hint.into();
        self
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    pub fn show(&mut self, ui: &mut Ui, text: &mut String) -> Response {
        let available_rect = ui.available_rect_before_wrap();
        let desired_size = Vec2::new(
            available_rect.width().max(100.0),
            available_rect.height().max(100.0),
        );

        let (rect, mut response) = ui.allocate_at_least(desired_size, egui::Sense::click());

        // Handle focus
        if response.clicked() {
            response.request_focus();
        }

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
        if response.has_focus() {
            let events = ui.input(|i| i.events.clone());
            for event in events {
                match event {
                    egui::Event::Text(new_text) => {
                        text.push_str(&new_text);
                        self.cursor_column += &new_text.chars().count();
                        std::println!("{0}", &self.cursor_column);
                        response.mark_changed();
                    }
                    egui::Event::Key {
                        key, pressed: true, ..
                    } => match key {
                        Key::Backspace => {
                            if !text.is_empty() {
                                text.pop();
                                response.mark_changed();
                            }
                        }
                        Key::Enter => {
                            text.push('\n');
                            response.mark_changed();
                        }
                        Key::Tab => {
                            text.push_str("    ");
                            response.mark_changed();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        let font_id = FontId::monospace(self.font_size);
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

        // Render lines
        self.render_lines(ui, text, &font_id, line_numbers_rect, content_rect);

        response
    }

    fn render_lines(
        &self,
        ui: &mut Ui,
        text: &str,
        font_id: &FontId,
        line_numbers_rect: Rect,
        content_rect: Rect,
    ) {
        if text.is_empty() {
            // Draw line number 1 and hint text
            ui.painter().text(
                Pos2::new(
                    line_numbers_rect.right() - ui.fonts(|f| f.glyph_width(font_id, '0')),
                    line_numbers_rect.top(),
                ),
                egui::Align2::RIGHT_TOP,
                "1",
                font_id.clone(),
                ui.visuals().weak_text_color(),
            );
            ui.painter().text(
                content_rect.left_top(),
                egui::Align2::LEFT_TOP,
                &self.hint_text,
                font_id.clone(),
                ui.visuals().weak_text_color(),
            );
        } else {
            let base_line_height = ui.fonts(|f| f.row_height(font_id));
            let mut current_y = line_numbers_rect.top();

            for (line_idx, line) in text.lines().enumerate() {
                let line_height = if let Some((texture, image_size)) = self.images.get(&line_idx) {
                    // Draw line text first
                    ui.painter().text(
                        Pos2::new(content_rect.left(), current_y),
                        egui::Align2::LEFT_TOP,
                        line,
                        font_id.clone(),
                        ui.visuals().text_color(),
                    );

                    // Draw image below the text with padding
                    let padding = 8.0;
                    let image_y = current_y + base_line_height + padding;
                    let image_rect = Rect::from_min_size(
                        Pos2::new(content_rect.left() + padding, image_y),
                        *image_size,
                    );
                    ui.painter().image(
                        texture.id(),
                        image_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );

                    // Total height: text + padding + image + padding
                    base_line_height + padding + image_size.y + padding
                } else {
                    base_line_height
                };

                // Draw line number (move this before the text drawing when no image)
                let line_number = format!("{}", line_idx + 1);
                ui.painter().text(
                    Pos2::new(
                        line_numbers_rect.right() - ui.fonts(|f| f.glyph_width(font_id, '0')),
                        current_y,
                    ),
                    egui::Align2::RIGHT_TOP,
                    &line_number,
                    font_id.clone(),
                    ui.visuals().weak_text_color(),
                );

                // Draw line text (only for lines without images, since image lines handle this above)
                if !self.images.contains_key(&line_idx) {
                    ui.painter().text(
                        Pos2::new(content_rect.left(), current_y),
                        egui::Align2::LEFT_TOP,
                        line,
                        font_id.clone(),
                        ui.visuals().text_color(),
                    );
                }

                // Draw cursor if this is the cursor line
                if line_idx == self.cursor_line {
                    let chars_to_cursor = self.cursor_column.min(line.len());
                    let x_offset = ui.fonts(|f| f.glyph_width(font_id, ' ')) * chars_to_cursor as f32;
                    let cursor_pos = Pos2::new(content_rect.left() + x_offset, current_y);

                    let blink = (ui.input(|i| i.time) * 2.0) as i32 % 2 == 0;
                    if blink {
                        ui.painter().vline(
                            cursor_pos.x,
                            cursor_pos.y..=(cursor_pos.y + base_line_height),
                            Stroke::new(1.0, ui.visuals().text_color()),
                        );
                    }
                }

                current_y += line_height;
            }

            // Handle trailing newline at end of text
            if text.ends_with('\n') {
                let extra_line = text.lines().count();
                ui.painter().text(
                    Pos2::new(
                        line_numbers_rect.right() - ui.fonts(|f| f.glyph_width(font_id, '0')),
                        current_y,
                    ),
                    egui::Align2::RIGHT_TOP,
                    &format!("{}", extra_line + 1),
                    font_id.clone(),
                    ui.visuals().weak_text_color(),
                );
            }
        }
    }

    pub fn add_image(&mut self, line_index: usize, texture: egui::TextureHandle, size: Vec2) {
        self.images.insert(line_index, (texture, size));
    }
}
