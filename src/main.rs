mod app;

use egui::Vec2;

use app::OobeApp;
use egui_extras::RetainedImage;

fn main() {
    let background_image = RetainedImage::from_image_bytes(
        "polkadot_background",
        include_bytes!("../assets/polkadot_background.png"),
    )
    .unwrap();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1512.0, 982.0)),
        ..Default::default()
    };

    eframe::run_native(
        "C4PIN OOBE",
        native_options,
        Box::new(|_| Box::new(OobeApp::new(background_image))),
    )
    .unwrap();
}
