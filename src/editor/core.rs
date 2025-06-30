use crate::models::{DocumentLine, LineElement};
use eframe::egui;

pub struct EditorCore {
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub scroll_offset: f32,
    pub selection_start: Option<(usize, usize)>,
    pub selection_end: Option<(usize, usize)>,
    pub cursor_blink_timer: f64,
    pub cursor_visible: bool,
}

impl Default for EditorCore {
    fn default() -> Self {
        Self {
            cursor_line: 0,
            cursor_col: 0,
            scroll_offset: 0.0,
            selection_start: None,
            selection_end: None,
            cursor_blink_timer: 0.0,
            cursor_visible: true,
        }
    }
}

impl EditorCore {
    pub fn insert_char(&mut self, c: char, lines: &mut Vec<DocumentLine>) -> bool {
        if self.cursor_line >= lines.len() {
            lines.push(DocumentLine::new());
        }

        if let Some(LineElement::Text(text)) = lines[self.cursor_line].elements.first_mut() {
            text.insert(self.cursor_col, c);
            self.cursor_col += 1;
        } else {
            // If current line doesn't start with text, insert a new text element
            let mut new_text = String::new();
            new_text.insert(0, c);
            lines[self.cursor_line]
                .elements
                .insert(0, LineElement::Text(new_text));
            self.cursor_col = 1;
        }
        self.clear_selection();
        true // Document modified
    }

    pub fn handle_enter(&mut self, lines: &mut Vec<DocumentLine>) -> bool {
        if self.cursor_line >= lines.len() {
            lines.push(DocumentLine::new());
        }

        // Split current line at cursor position
        if let Some(LineElement::Text(text)) = lines[self.cursor_line].elements.first_mut() {
            let remaining_text = text.split_off(self.cursor_col);

            // Create new line with remaining text
            let new_line = DocumentLine {
                elements: vec![LineElement::Text(remaining_text)],
            };

            self.cursor_line += 1;
            lines.insert(self.cursor_line, new_line);
            self.cursor_col = 0;
        } else {
            // Just add a new empty line
            self.cursor_line += 1;
            lines.insert(self.cursor_line, DocumentLine::new());
            self.cursor_col = 0;
        }
        true // Document modified
    }

    pub fn handle_backspace(&mut self, lines: &mut Vec<DocumentLine>) -> bool {
        if self.cursor_col > 0 {
            // Delete character in current line
            if let Some(LineElement::Text(text)) = lines[self.cursor_line].elements.first_mut() {
                if self.cursor_col <= text.len() {
                    text.remove(self.cursor_col - 1);
                    self.cursor_col -= 1;
                    return true; // Document modified
                }
            }
        } else if self.cursor_line > 0 {
            // Merge with previous line
            let current_line = lines.remove(self.cursor_line);
            self.cursor_line -= 1;

            if let Some(LineElement::Text(prev_text)) = lines[self.cursor_line].elements.first_mut()
            {
                self.cursor_col = prev_text.len();
                if let Some(LineElement::Text(current_text)) = current_line.elements.first() {
                    prev_text.push_str(current_text);
                }
            }
            return true; // Document modified
        }
        false // No modification
    }

    pub fn move_cursor_left(&mut self, lines: &[DocumentLine]) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = lines[self.cursor_line].text_content().len();
        }
        self.reset_cursor_blink();
    }

    pub fn move_cursor_right(&mut self, lines: &[DocumentLine]) {
        let current_line_len = lines
            .get(self.cursor_line)
            .map(|line| line.text_content().len())
            .unwrap_or(0);

        if self.cursor_col < current_line_len {
            self.cursor_col += 1;
        } else if self.cursor_line < lines.len() - 1 {
            self.cursor_line += 1;
            self.cursor_col = 0;
        }
        self.reset_cursor_blink();
    }

    pub fn move_cursor_up(&mut self, lines: &[DocumentLine]) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            let line_len = lines[self.cursor_line].text_content().len();
            self.cursor_col = self.cursor_col.min(line_len);
        }
        self.reset_cursor_blink();
    }

    pub fn move_cursor_down(&mut self, lines: &[DocumentLine]) {
        if self.cursor_line < lines.len() - 1 {
            self.cursor_line += 1;
            let line_len = lines[self.cursor_line].text_content().len();
            self.cursor_col = self.cursor_col.min(line_len);
        }
        self.reset_cursor_blink();
    }

    pub fn handle_input(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        lines: &mut Vec<DocumentLine>,
    ) -> bool {
        let mut modified = false;

        // Update cursor blink
        self.update_cursor_blink(ctx);

        // Handle text input
        ui.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Text(text) => {
                        // Delete selection if any before inserting new text
                        if self.has_selection() {
                            if self.delete_selection(lines) {
                                modified = true;
                            }
                        }

                        for c in text.chars() {
                            if c.is_control() {
                                continue;
                            }
                            if self.insert_char(c, lines) {
                                modified = true;
                            }
                        }
                        self.reset_cursor_blink();
                    }
                    egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        match key {
                            egui::Key::Enter => {
                                if self.has_selection() {
                                    if self.delete_selection(lines) {
                                        modified = true;
                                    }
                                }
                                if self.handle_enter(lines) {
                                    modified = true;
                                }
                                self.reset_cursor_blink();
                            }
                            egui::Key::Backspace => {
                                if self.has_selection() {
                                    if self.delete_selection(lines) {
                                        modified = true;
                                    }
                                } else if self.handle_backspace(lines) {
                                    modified = true;
                                }
                                self.reset_cursor_blink();
                            }
                            egui::Key::Delete => {
                                if self.has_selection() {
                                    if self.delete_selection(lines) {
                                        modified = true;
                                    }
                                } else {
                                    // Handle delete key (delete character to the right)
                                    if self.cursor_line < lines.len() {
                                        let mut should_merge = false;

                                        // First, try to delete a character within the line
                                        if let Some(crate::models::LineElement::Text(text)) =
                                            lines[self.cursor_line].elements.first_mut()
                                        {
                                            if self.cursor_col < text.len() {
                                                text.remove(self.cursor_col);
                                                modified = true;
                                            } else if self.cursor_line < lines.len() - 1 {
                                                should_merge = true;
                                            }
                                        }

                                        // If we need to merge lines, handle it separately
                                        if should_merge {
                                            let next_line = lines.remove(self.cursor_line + 1);
                                            if let Some(crate::models::LineElement::Text(
                                                current_text,
                                            )) = lines[self.cursor_line].elements.first_mut()
                                            {
                                                if let Some(crate::models::LineElement::Text(
                                                    next_text,
                                                )) = next_line.elements.first()
                                                {
                                                    current_text.push_str(next_text);
                                                    modified = true;
                                                }
                                            }
                                        }
                                    }
                                }
                                self.reset_cursor_blink();
                            }
                            egui::Key::ArrowLeft => {
                                if modifiers.shift {
                                    if self.selection_start.is_none() {
                                        self.start_selection();
                                    }
                                } else {
                                    self.clear_selection();
                                }
                                self.move_cursor_left(lines);
                                if modifiers.shift {
                                    self.update_selection();
                                }
                            }
                            egui::Key::ArrowRight => {
                                if modifiers.shift {
                                    if self.selection_start.is_none() {
                                        self.start_selection();
                                    }
                                } else {
                                    self.clear_selection();
                                }
                                self.move_cursor_right(lines);
                                if modifiers.shift {
                                    self.update_selection();
                                }
                            }
                            egui::Key::ArrowUp => {
                                if modifiers.shift {
                                    if self.selection_start.is_none() {
                                        self.start_selection();
                                    }
                                } else {
                                    self.clear_selection();
                                }
                                self.move_cursor_up(lines);
                                if modifiers.shift {
                                    self.update_selection();
                                }
                            }
                            egui::Key::ArrowDown => {
                                if modifiers.shift {
                                    if self.selection_start.is_none() {
                                        self.start_selection();
                                    }
                                } else {
                                    self.clear_selection();
                                }
                                self.move_cursor_down(lines);
                                if modifiers.shift {
                                    self.update_selection();
                                }
                            }
                            egui::Key::A if modifiers.ctrl => {
                                // Select all
                                self.selection_start = Some((0, 0));
                                if let Some(last_line) = lines.last() {
                                    self.selection_end =
                                        Some((lines.len() - 1, last_line.text_content().len()));
                                }
                            }
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

        modified
    }

    pub fn reset(&mut self) {
        self.cursor_line = 0;
        self.cursor_col = 0;
        self.scroll_offset = 0.0;
        self.selection_start = None;
        self.selection_end = None;
        self.cursor_blink_timer = 0.0;
        self.cursor_visible = true;
    }

    pub fn reset_cursor_blink(&mut self) {
        self.cursor_blink_timer = 0.0;
        self.cursor_visible = true;
    }

    pub fn update_cursor_blink(&mut self, ctx: &egui::Context) {
        self.cursor_blink_timer += ctx.input(|i| i.stable_dt) as f64;
        if self.cursor_blink_timer >= 0.5 {
            self.cursor_visible = !self.cursor_visible;
            self.cursor_blink_timer = 0.0;
            ctx.request_repaint();
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }

    pub fn start_selection(&mut self) {
        self.selection_start = Some((self.cursor_line, self.cursor_col));
        self.selection_end = Some((self.cursor_line, self.cursor_col));
    }

    pub fn update_selection(&mut self) {
        if self.selection_start.is_some() {
            self.selection_end = Some((self.cursor_line, self.cursor_col));
        }
    }

    pub fn has_selection(&self) -> bool {
        self.selection_start.is_some()
            && self.selection_end.is_some()
            && self.selection_start != self.selection_end
    }

    pub fn delete_selection(&mut self, lines: &mut Vec<DocumentLine>) -> bool {
        if !self.has_selection() {
            return false;
        }

        let (start_line, start_col) = self.selection_start.unwrap();
        let (end_line, end_col) = self.selection_end.unwrap();

        // Ensure start is before end
        let (start_line, start_col, end_line, end_col) =
            if start_line < end_line || (start_line == end_line && start_col < end_col) {
                (start_line, start_col, end_line, end_col)
            } else {
                (end_line, end_col, start_line, start_col)
            };

        if start_line == end_line {
            // Single line selection
            if let Some(crate::models::LineElement::Text(text)) =
                lines[start_line].elements.first_mut()
            {
                text.drain(start_col..end_col);
                self.cursor_line = start_line;
                self.cursor_col = start_col;
            }
        } else {
            // Multi-line selection
            // Remove complete lines in between
            if end_line > start_line + 1 {
                lines.drain((start_line + 1)..end_line);
            }

            // Handle start and end lines
            let remaining_end = if start_line + 1 < lines.len() {
                if let Some(crate::models::LineElement::Text(end_text)) =
                    lines[start_line + 1].elements.first()
                {
                    end_text.chars().skip(end_col).collect::<String>()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            if let Some(crate::models::LineElement::Text(start_text)) =
                lines[start_line].elements.first_mut()
            {
                let remaining_start = start_text.chars().take(start_col).collect::<String>();
                *start_text = remaining_start + &remaining_end;

                self.cursor_line = start_line;
                self.cursor_col = start_col;
            }

            // Remove the end line
            if lines.len() > start_line + 1 {
                lines.remove(start_line + 1);
            }
        }

        self.clear_selection();
        true
    }
}
