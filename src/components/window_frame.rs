use eframe::egui::{self, ViewportCommand};

// Window frame constants
const TITLE_BAR_HEIGHT: f32 = 32.0;
const WINDOW_BUTTON_HEIGHT: f32 = 24.0;
const WINDOW_BUTTON_SPACING: f32 = 8.0;
const TITLE_BAR_PADDING: f32 = 8.0;
const CONTENT_PADDING: f32 = 4.0;
const WINDOW_BORDER_WIDTH: f32 = 1.0;
const TITLE_FONT_SIZE: f32 = 20.0;

pub struct CustomWindowFrame;

impl CustomWindowFrame {
    pub fn show(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
        use egui::{CentralPanel, UiBuilder};

        let panel_frame = egui::Frame::new()
            .fill(ctx.style().visuals.window_fill())
            .stroke(ctx.style().visuals.widgets.noninteractive.fg_stroke)
            .outer_margin(WINDOW_BORDER_WIDTH);

        CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + TITLE_BAR_HEIGHT;
                rect
            };

            TitleBar::show(ui, title_bar_rect, title);

            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(CONTENT_PADDING);

            let mut content_ui = ui.new_child(UiBuilder::new().max_rect(content_rect));
            add_contents(&mut content_ui);
        });
    }
}

struct TitleBar;

impl TitleBar {
    fn show(ui: &mut egui::Ui, title_bar_rect: egui::Rect, title: &str) {
        use egui::{Align2, FontId, Id, PointerButton, Sense, UiBuilder, vec2};

        let painter = ui.painter();

        let title_bar_response = ui.interact(
            title_bar_rect,
            Id::new("title_bar"),
            Sense::click_and_drag(),
        );

        // Paint the title
        painter.text(
            title_bar_rect.center(),
            Align2::CENTER_CENTER,
            title,
            FontId::proportional(TITLE_FONT_SIZE),
            ui.style().visuals.text_color(),
        );

        // Paint the line under the title
        painter.line_segment(
            [
                title_bar_rect.left_bottom() + vec2(WINDOW_BORDER_WIDTH, 0.0),
                title_bar_rect.right_bottom() + vec2(-WINDOW_BORDER_WIDTH, 0.0),
            ],
            ui.visuals().widgets.noninteractive.bg_stroke,
        );

        // Handle interactions
        Self::handle_interactions(ui, &title_bar_response);
        Self::show_window_controls(ui, title_bar_rect);
    }

    fn handle_interactions(ui: &mut egui::Ui, title_bar_response: &egui::Response) {
        if title_bar_response.double_clicked() {
            let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
        }

        if title_bar_response.drag_started_by(egui::PointerButton::Primary) {
            ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
        }
    }

    fn show_window_controls(ui: &mut egui::Ui, title_bar_rect: egui::Rect) {
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(title_bar_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.visuals_mut().button_frame = false;
                ui.add_space(TITLE_BAR_PADDING);
                WindowControls::show(ui);
            },
        );
    }
}

struct WindowControls;

impl WindowControls {
    fn show(ui: &mut egui::Ui) {
        use egui::{Button, RichText};

        // Close button
        let close_response = ui
            .add(Button::new(RichText::new("×").size(WINDOW_BUTTON_HEIGHT)))
            .on_hover_text("Close the window");
        if close_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        }

        ui.add_space(WINDOW_BUTTON_SPACING);

        // Maximize/Restore button
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        let (icon, tooltip) = if is_maximized {
            ("◱", "Restore window")
        } else {
            ("□", "Maximize window")
        };

        let maximize_response = ui
            .add(Button::new(RichText::new(icon).size(WINDOW_BUTTON_HEIGHT)))
            .on_hover_text(tooltip);
        if maximize_response.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
        }

        ui.add_space(WINDOW_BUTTON_SPACING);

        // Minimize button
        let minimize_response = ui
            .add(Button::new(RichText::new("−").size(WINDOW_BUTTON_HEIGHT)))
            .on_hover_text("Minimize the window");
        if minimize_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
        }
    }
}
