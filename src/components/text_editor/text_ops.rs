use crate::components::text_editor::cursor_position::CursorPosition;

pub fn insert_newline_at_position(text: &mut String, cursor_position: CursorPosition) {
    insert_text_at_position(text, cursor_position, "\n");
}

pub fn insert_text_at_position(text: &mut String, cursor_position: CursorPosition, new_text: &str) {
    let pos = get_byte_position(text, cursor_position);
    text.insert_str(pos, new_text);
}

pub fn delete_char_at_position(text: &mut String, cursor_position: CursorPosition) {
    let pos = get_byte_position(text, cursor_position);
    if let Some((char_start, _)) = text.char_indices().nth(pos.saturating_sub(1)) {
        text.remove(char_start);
    }
}

fn get_byte_position(text: &str, cursor_position: CursorPosition) -> usize {
    let lines: Vec<&str> = text.lines().collect();
    let mut byte_pos = 0;

    for (i, line) in lines.iter().enumerate() {
        if i == cursor_position.get_line_position() {
            byte_pos += line
                .chars()
                .take(cursor_position.get_column_position())
                .map(|c| c.len_utf8())
                .sum::<usize>();
            break;
        }
        byte_pos += line.len() + 1;
    }
    byte_pos
}
