use crate::components::text_editor::editor::{IMAGE_PADDING, TextEditorImageMap};

pub fn extract_image_id(line: &str) -> Option<usize> {
    if let Some(start) = line.find("[image(") {
        if let Some(end) = line[start + 7..].find(")]") {
            let id_str = &line[start + 7..start + 7 + end];
            return id_str.parse::<usize>().ok();
        }
    }
    None
}

pub fn calculate_line_height(
    line: &str,
    base_line_height: f32,
    images: &TextEditorImageMap,
) -> f32 {
    if let Some(image_id) = extract_image_id(line) {
        if let Some((_, image_size)) = images.get(&image_id) {
            base_line_height + IMAGE_PADDING + image_size.y + IMAGE_PADDING
        } else {
            base_line_height
        }
    } else {
        base_line_height
    }
}
