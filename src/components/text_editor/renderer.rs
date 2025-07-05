use super::TextEditor;
use egui::{Color32, FontId, Pos2, Rect, Stroke, Ui, Vec2};

pub struct TextEditorRenderer;

impl TextEditorRenderer {
    // fn render_lines(
    //     &mut self,
    //     ui: &mut Ui,
    //     text: &str,
    //     font_id: &FontId,
    //     line_numbers_rect: Rect,
    //     content_rect: Rect,
    // ) {
    //     if text.is_empty() {
    //         self.render_empty_editor(ui, font_id, line_numbers_rect, content_rect);
    //     } else {
    //         self.render_text_content(ui, text, font_id, line_numbers_rect, content_rect);
    //     }
    // }
    // 
    // fn render_empty_editor(
    //     &self,
    //     ui: &mut Ui,
    //     font_id: &FontId,
    //     line_numbers_rect: Rect,
    //     content_rect: Rect,
    // ) {
    //     ui.painter().text(
    //         Pos2::new(
    //             line_numbers_rect.right() - ui.fonts(|f| f.glyph_width(font_id, '0')),
    //             line_numbers_rect.top(),
    //         ),
    //         egui::Align2::RIGHT_TOP,
    //         "1",
    //         font_id.clone(),
    //         ui.visuals().weak_text_color(),
    //     );
    //     ui.painter().text(
    //         content_rect.left_top(),
    //         egui::Align2::LEFT_TOP,
    //         &self.hint_text,
    //         font_id.clone(),
    //         ui.visuals().weak_text_color(),
    //     );
    // }
    // 
    // fn render_text_content(
    //     &mut self,
    //     ui: &mut Ui,
    //     text: &str,
    //     font_id: &FontId,
    //     line_numbers_rect: Rect,
    //     content_rect: Rect,
    // ) {
    //     let base_line_height = ui.fonts(|f| f.row_height(font_id));
    //     let mut current_y = line_numbers_rect.top();
    // 
    //     for (line_idx, line) in text.lines().enumerate() {
    //         let mut line_height = self.calculate_line_height(line, base_line_height);
    // 
    //         // If there's an image, draw it and change the line_height
    //         if let Some((texture, image_size)) = self
    //             .extract_image_id(line)
    //             .and_then(|id| self.images.get(&id))
    //         {
    //             line_height = base_line_height + IMAGE_PADDING + image_size.y + IMAGE_PADDING;
    // 
    //             // Draw image below the text with padding
    //             let image_y = current_y + base_line_height + IMAGE_PADDING;
    //             let image_rect = Rect::from_min_size(
    //                 Pos2::new(content_rect.left() + IMAGE_PADDING, image_y),
    //                 *image_size,
    //             );
    //             ui.painter().image(
    //                 texture.id(),
    //                 image_rect,
    //                 Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
    //                 Color32::WHITE,
    //             );
    //         }
    // 
    //         // Draw line number (move this before the text drawing when no image)
    //         let line_number = format!("{}", line_idx + 1);
    //         ui.painter().text(
    //             Pos2::new(
    //                 line_numbers_rect.right() - ui.fonts(|f| f.glyph_width(font_id, '0')),
    //                 current_y,
    //             ),
    //             egui::Align2::RIGHT_TOP,
    //             &line_number,
    //             font_id.clone(),
    //             ui.visuals().weak_text_color(),
    //         );
    // 
    //         // Draw line text
    //         ui.painter().text(
    //             Pos2::new(content_rect.left(), current_y),
    //             egui::Align2::LEFT_TOP,
    //             line,
    //             font_id.clone(),
    //             ui.visuals().text_color(),
    //         );
    // 
    //         // Draw cursor if this is the cursor line
    //         if line_idx == self.cursor_line {
    //             let chars_to_cursor = self.cursor_column.min(line.chars().count());
    //             let text_before_cursor = line.chars().take(chars_to_cursor).collect::<String>();
    // 
    //             let x_offset = ui.fonts(|f| {
    //                 f.layout_no_wrap(text_before_cursor, font_id.clone(), Color32::WHITE).size().x
    //             });
    // 
    //             let cursor_pos = Pos2::new(content_rect.left() + x_offset, current_y);
    // 
    //             ui.painter().vline(
    //                 cursor_pos.x,
    //                 cursor_pos.y..=(cursor_pos.y + base_line_height),
    //                 Stroke::new(1.0, ui.visuals().text_color()),
    //             );
    //         }
    // 
    //         current_y += line_height;
    //     }
    // 
    //     // Handle trailing newline at end of text
    //     if text.ends_with('\n') {
    //         let extra_line = text.lines().count();
    //         ui.painter().text(
    //             Pos2::new(
    //                 line_numbers_rect.right() - ui.fonts(|f| f.glyph_width(font_id, '0')),
    //                 current_y,
    //             ),
    //             egui::Align2::RIGHT_TOP,
    //             &format!("{}", extra_line + 1),
    //             font_id.clone(),
    //             ui.visuals().weak_text_color(),
    //         );
    //     }
    // }

}