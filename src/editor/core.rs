use crate::models::{DocumentLine, LineElement};
use eframe::egui;

pub struct EditorCore {
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub scroll_offset: f32,
}

impl Default for EditorCore {
    fn default() -> Self {
        Self {
            cursor_line: 0,
            cursor_col: 0,
            scroll_offset: 0.0,
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
            lines[self.cursor_line].elements.insert(0, LineElement::Text(new_text));
            self.cursor_col = 1;
        }
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
            
            if let Some(LineElement::Text(prev_text)) = lines[self.cursor_line].elements.first_mut() {
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
    }
    
    pub fn move_cursor_right(&mut self, lines: &[DocumentLine]) {
        let current_line_len = lines.get(self.cursor_line)
            .map(|line| line.text_content().len())
            .unwrap_or(0);
            
        if self.cursor_col < current_line_len {
            self.cursor_col += 1;
        } else if self.cursor_line < lines.len() - 1 {
            self.cursor_line += 1;
            self.cursor_col = 0;
        }
    }
    
    pub fn move_cursor_up(&mut self, lines: &[DocumentLine]) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            let line_len = lines[self.cursor_line].text_content().len();
            self.cursor_col = self.cursor_col.min(line_len);
        }
    }
    
    pub fn move_cursor_down(&mut self, lines: &[DocumentLine]) {
        if self.cursor_line < lines.len() - 1 {
            self.cursor_line += 1;
            let line_len = lines[self.cursor_line].text_content().len();
            self.cursor_col = self.cursor_col.min(line_len);
        }
    }
    
    pub fn handle_input(&mut self, ui: &mut egui::Ui, lines: &mut Vec<DocumentLine>) -> bool {
        let mut modified = false;
        
        // Handle text input
        ui.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Text(text) => {
                        for c in text.chars() {
                            if c.is_control() {
                                continue;
                            }
                            if self.insert_char(c, lines) {
                                modified = true;
                            }
                        }
                    }
                    egui::Event::Key { key, pressed: true, modifiers: _, .. } => {
                        match key {
                            egui::Key::Enter => {
                                if self.handle_enter(lines) {
                                    modified = true;
                                }
                            }
                            egui::Key::Backspace => {
                                if self.handle_backspace(lines) {
                                    modified = true;
                                }
                            }
                            egui::Key::ArrowLeft => self.move_cursor_left(lines),
                            egui::Key::ArrowRight => self.move_cursor_right(lines),
                            egui::Key::ArrowUp => self.move_cursor_up(lines),
                            egui::Key::ArrowDown => self.move_cursor_down(lines),
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
    }
}