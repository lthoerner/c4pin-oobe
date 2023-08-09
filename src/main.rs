mod app;

use egui::Vec2;

use app::OobeApp;

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(640.0, 360.0)),
        ..Default::default()
    };

    eframe::run_native(
        "OOBE",
        native_options,
        Box::new(|_| Box::<OobeApp>::default()),
    )
    .unwrap();
}
