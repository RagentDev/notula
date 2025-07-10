use std::cmp::min;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    line: usize,
    column: usize,
}

impl CursorPosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn get_line_position(self) -> usize {
        self.line
    }

    pub fn get_column_position(self) -> usize {
        self.column
    }

    pub fn move_up(&mut self, text: &str) {
        if self.line > 0 {
            self.line -= 1;
            self.clamp_column(text);
        }
    }

    pub fn move_down(&mut self, text: &str) {
        let line_count = self.get_max_line_index(text);
        self.line = min(self.line + 1, line_count);
        self.clamp_column(text);
    }

    pub fn move_left(&mut self, text: &str) {
        if self.column > 0 {
            self.column -= 1;
        } else if self.line > 0 {
            self.move_up(text);
            self.move_to_line_end(text);
        }
    }

    pub fn move_right(&mut self, text: &str) {
        let line_count = self.get_max_line_index(text);

        let lines: Vec<&str> = text.lines().collect();
        if let Some(line) = lines.get(self.line) {
            let character_count = line.chars().count();
            if self.column == character_count && self.line < line_count {
                self.move_down(text);
                self.move_to_line_start();
            } else {
                self.column = min(self.column + 1, character_count);
            }
        }
    }

    pub fn move_right_by(&mut self, amount: usize) {
        self.column += amount;
    }

    pub fn move_to_line_and_column(&mut self, line: usize, column: usize, text: &str) {
        let line_count = self.get_max_line_index(text);
        self.line = min(line, line_count);

        let lines: Vec<&str> = text.lines().collect();
        if let Some(line) = lines.get(self.line) {
            let character_count = line.chars().count();
            self.column = min(column, character_count);
        }
    }

    pub fn move_to_line_end(&mut self, text: &str) {
        let lines: Vec<&str> = text.lines().collect();
        if let Some(line) = lines.get(self.line) {
            self.column = line.chars().count();
        }
    }

    pub fn move_to_line_start(&mut self) {
        self.column = 0;
    }

    fn get_max_line_index(&mut self, text: &str) -> usize {
        let base_lines = text.lines().count();
        if text.ends_with('\n') && !text.is_empty() {
            base_lines
        } else {
            base_lines.saturating_sub(1)
        }
    }

    fn clamp_column(&mut self, text: &str) {
        let lines: Vec<&str> = text.lines().collect();
        if let Some(line) = lines.get(self.line) {
            self.column = self.column.min(line.chars().count());
        }
    }
}
