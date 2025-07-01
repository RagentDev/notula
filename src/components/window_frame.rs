use crate::icons::Icons;
use eframe::egui::{self, ViewportCommand};
use eframe::emath::Vec2;
use eframe::epaint::Color32;
use egui::{Button, Response};
use crate::assets::AssetManager;

// Window frame constants
const RESIZE_HANDLE_SIZE: f32 = 4.0;
const TITLE_BAR_HEIGHT: f32 = 32.0;
const WINDOW_BUTTON_WIDTH: f32 = 32.0;
const WINDOW_BUTTON_HEIGHT: f32 = 24.0;
const WINDOW_BUTTON_SPACING: f32 = 8.0;
const TITLE_BAR_PADDING: f32 = 8.0;
const CONTENT_PADDING: f32 = 4.0;
const WINDOW_BORDER_WIDTH: f32 = 1.0;
const TITLE_FONT_SIZE: f32 = 20.0;

pub struct CustomWindowFrame;

impl CustomWindowFrame {
    pub fn show(
        ctx: &egui::Context,
        title: &str,
        assets: &AssetManager,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) {
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

            TitleBar::show(ui, title_bar_rect, title, assets);

            let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            if !is_maximized {
                Self::handle_resize(ui, app_rect);
            }

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

    fn handle_resize(ui: &mut egui::Ui, window_rect: egui::Rect) {
        use egui::{CursorIcon, Id, Sense};

        let resize_size = RESIZE_HANDLE_SIZE;

        // Define resize areas
        let resize_areas = [
            // Corners
            (
                window_rect.left_top(),
                resize_size,
                resize_size,
                CursorIcon::ResizeNwSe,
                egui::Vec2::new(-1.0, -1.0),
            ), // Top-left
            (
                egui::Pos2::new(window_rect.right() - resize_size, window_rect.top()),
                resize_size,
                resize_size,
                CursorIcon::ResizeNeSw,
                egui::Vec2::new(1.0, -1.0),
            ), // Top-right
            (
                egui::Pos2::new(window_rect.left(), window_rect.bottom() - resize_size),
                resize_size,
                resize_size,
                CursorIcon::ResizeNeSw,
                egui::Vec2::new(-1.0, 1.0),
            ), // Bottom-left
            (
                egui::Pos2::new(
                    window_rect.right() - resize_size,
                    window_rect.bottom() - resize_size,
                ),
                resize_size,
                resize_size,
                CursorIcon::ResizeNwSe,
                egui::Vec2::new(1.0, 1.0),
            ), // Bottom-right
            // Edges
            (
                egui::Pos2::new(window_rect.left(), window_rect.top() + resize_size),
                resize_size,
                window_rect.height() - 2.0 * resize_size,
                CursorIcon::ResizeHorizontal,
                egui::Vec2::new(-1.0, 0.0),
            ), // Left edge
            (
                egui::Pos2::new(
                    window_rect.right() - resize_size,
                    window_rect.top() + resize_size,
                ),
                resize_size,
                window_rect.height() - 2.0 * resize_size,
                CursorIcon::ResizeHorizontal,
                egui::Vec2::new(1.0, 0.0),
            ), // Right edge
            (
                egui::Pos2::new(window_rect.left() + resize_size, window_rect.top()),
                window_rect.width() - 2.0 * resize_size,
                resize_size,
                CursorIcon::ResizeVertical,
                egui::Vec2::new(0.0, -1.0),
            ), // Top edge
            (
                egui::Pos2::new(
                    window_rect.left() + resize_size,
                    window_rect.bottom() - resize_size,
                ),
                window_rect.width() - 2.0 * resize_size,
                resize_size,
                CursorIcon::ResizeVertical,
                egui::Vec2::new(0.0, 1.0),
            ), // Bottom edge
        ];

        for (i, (pos, width, height, cursor_icon, direction)) in resize_areas.iter().enumerate() {
            let resize_rect = egui::Rect::from_min_size(*pos, egui::Vec2::new(*width, *height));

            let response = ui.interact(
                resize_rect,
                Id::new(format!("resize_handle_{}", i)),
                Sense::click_and_drag(),
            );

            if response.hovered() {
                ui.ctx().set_cursor_icon(*cursor_icon);
            }

            if response.drag_started() {
                // Start resize operation
                ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
                    match (direction.x, direction.y) {
                        (-1.0, -1.0) => egui::viewport::ResizeDirection::NorthWest,
                        (0.0, -1.0) => egui::viewport::ResizeDirection::North,
                        (1.0, -1.0) => egui::viewport::ResizeDirection::NorthEast,
                        (-1.0, 0.0) => egui::viewport::ResizeDirection::West,
                        (1.0, 0.0) => egui::viewport::ResizeDirection::East,
                        (-1.0, 1.0) => egui::viewport::ResizeDirection::SouthWest,
                        (0.0, 1.0) => egui::viewport::ResizeDirection::South,
                        (1.0, 1.0) => egui::viewport::ResizeDirection::SouthEast,
                        _ => egui::viewport::ResizeDirection::SouthEast,
                    },
                ));
            }
        }
    }
}

struct TitleBar;

impl TitleBar {
    fn show(ui: &mut egui::Ui, title_bar_rect: egui::Rect, title: &str, assets: &AssetManager) {
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
        Self::show_window_controls(ui, title_bar_rect, assets);
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

    fn show_window_controls(ui: &mut egui::Ui, title_bar_rect: egui::Rect, assets: &AssetManager) {
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(title_bar_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.visuals_mut().button_frame = false;
                ui.add_space(TITLE_BAR_PADDING);
                WindowControls::show(ui, assets);
            },
        );
    }
}

struct WindowControls;

impl WindowControls {
    fn show(ui: &mut egui::Ui, assets: &AssetManager) {
        // Close button
        let close_button = Self::create_button(
            ui,
            &assets.icons.close,
            "Close the window",
            Color32::from_rgba_unmultiplied(220, 53, 69, 50),
        );

        if close_button.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        }

        ui.add_space(WINDOW_BUTTON_SPACING);

        // Maximize/Restore button
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        let (icon, tooltip) = if is_maximized {
            (&assets.icons.maximize, "Restore window")
        } else {
            (&assets.icons.restore, "Maximize window")
        };

        let maximize_button = Self::create_button(
            ui,
            icon,
            tooltip,
            Color32::from_rgba_unmultiplied(200, 200, 200, 5),
        );

        if maximize_button.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
        }

        ui.add_space(WINDOW_BUTTON_SPACING);

        // Minimize button
        let minimize_button = Self::create_button(
            ui,
            &assets.icons.minimize,
            "Minimize the window",
            Color32::from_rgba_unmultiplied(200, 200, 200, 5),
        );

        if minimize_button.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
        }
    }

    fn create_button(
        ui: &mut egui::Ui,
        image: &egui::ImageSource<'static>,
        hover_text: &str,
        highlight_color: Color32,
    ) -> Response {
        let button_size = Vec2::new(WINDOW_BUTTON_WIDTH, WINDOW_BUTTON_HEIGHT);

        let button = ui
            .add_sized(button_size, Button::image(image.clone()))
            .on_hover_text(hover_text);

        if button.hovered() {
            ui.painter().rect_filled(
                button.rect,
                ui.style().visuals.widgets.noninteractive.corner_radius,
                highlight_color,
            );
        }

        button
    }
}
