use crate::models::{DocumentLine, LineElement};
use eframe::egui;

pub struct StatusBar;

impl StatusBar {
    pub fn render(ui: &mut egui::Ui, lines: &[DocumentLine]) {
        ui.horizontal(|ui| {
            let (line_count, char_count) = Self::get_stats(lines);
            ui.label(format!("Lines: {}", line_count));
            ui.separator();
            ui.label(format!("Characters: {}", char_count));
        });
    }

    fn get_stats(lines: &[DocumentLine]) -> (usize, usize) {
        let line_count = lines.len();
        let char_count = lines
            .iter()
            .map(|line| line.text_content().chars().count())
            .sum::<usize>()
            + lines
                .iter()
                .map(|line| {
                    line.elements
                        .iter()
                        .filter(|e| matches!(e, LineElement::Image { .. }))
                        .count()
                })
                .sum::<usize>();
        (line_count, char_count)
    }
}
