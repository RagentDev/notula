macro_rules! asset_path {
    ($path:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path)
    };
}

pub struct Icons {
    pub close: egui::ImageSource<'static>,
    pub minimize: egui::ImageSource<'static>,
    pub maximize: egui::ImageSource<'static>,
    pub restore: egui::ImageSource<'static>,
}

impl Default for Icons {
    fn default() -> Self {
        Self {
            close: egui::include_image!("assets/icons/close.svg"),
            minimize: egui::include_image!("assets/icons/minimize.svg"),
            maximize: egui::include_image!("assets/icons/maximize.svg"),
            restore: egui::include_image!("assets/icons/restore.svg"),
        }
    }
}