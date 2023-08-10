mod app;

use eframe::Theme;
use egui::Vec2;

use app::OobeApp;

fn main() {
    let native_options = eframe::NativeOptions {
        default_theme: Theme::Dark,
        initial_window_size: Some(Vec2::new(1512.0, 982.0)),
        ..Default::default()
    };

    eframe::run_native(
        "C4PIN OOBE",
        native_options,
        Box::new(|cc| Box::new(OobeApp::new(cc))),
    )
    .unwrap();
}
