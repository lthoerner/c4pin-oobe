fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "OOBE",
        native_options,
        Box::new(|_| Box::new(OobeApp::new())),
    )
    .unwrap();
}

#[derive(Default)]
struct OobeApp {}

impl OobeApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for OobeApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OOBE");
        });
    }
}
