use eframe::egui;

pub struct MenuActions {
    pub new_file: bool,
    pub open_file: bool,
    pub save_file: bool,
    pub save_as: bool,
    pub exit: bool,
    pub paste_image: bool,
    pub insert_sample_image: bool,
}

impl Default for MenuActions {
    fn default() -> Self {
        Self {
            new_file: false,
            open_file: false,
            save_file: false,
            save_as: false,
            exit: false,
            paste_image: false,
            insert_sample_image: false,
        }
    }
}

pub struct MenuBar;

impl MenuBar {
    pub fn render(ui: &mut egui::Ui, ctx: &egui::Context) -> MenuActions {
        let mut actions = MenuActions::default();

        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    actions.new_file = true;
                    ui.close_menu();
                }
                if ui.button("Open...").clicked() {
                    actions.open_file = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Save").clicked() {
                    actions.save_file = true;
                    ui.close_menu();
                }
                if ui.button("Save As...").clicked() {
                    actions.save_as = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    actions.exit = true;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    // TODO: Implement undo
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Cut").clicked() {
                    // TODO: Implement cut
                    ui.close_menu();
                }
                if ui.button("Copy").clicked() {
                    // TODO: Implement copy
                    ui.close_menu();
                }
                if ui.button("Paste").clicked() {
                    // TODO: Implement text paste
                    ui.close_menu();
                }
                if ui.button("Paste Image (Ctrl+V)").clicked() {
                    actions.paste_image = true;
                    ui.close_menu();
                }
                if ui.button("Insert Sample Image").clicked() {
                    actions.insert_sample_image = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Select All").clicked() {
                    // TODO: Implement select all
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Find...").clicked() {
                    // TODO: Implement find
                    ui.close_menu();
                }
            });

            ui.menu_button("View", |ui| {
                if ui.button("Word Wrap").clicked() {
                    // TODO: Implement word wrap toggle
                    ui.close_menu();
                }
                if ui.button("Font...").clicked() {
                    // TODO: Implement font selection
                    ui.close_menu();
                }
            });
        });

        actions
    }
}
