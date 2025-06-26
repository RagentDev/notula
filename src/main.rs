use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you want logging)

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Captum"),
        ..Default::default()
    };

    eframe::run_native(
        "Captum",
        options,
        Box::new(|_cc| Box::<CaptumApp>::default()),
    )
}

struct CaptumApp {
    text: String,
}

impl Default for CaptumApp {
    fn default() -> Self {
        Self {
            text: "Hello, world!".to_owned(),
        }
    }
}

impl eframe::App for CaptumApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Captum - Lightweight Notepad");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Text editor:");
                if ui.button("Clear").clicked() {
                    self.text.clear();
                }
            });

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.text_edit_multiline(&mut self.text);
                });
        });
    }
}