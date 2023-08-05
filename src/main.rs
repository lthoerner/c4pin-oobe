use egui_extras::RetainedImage;

fn main() {
    let firefox_icon = RetainedImage::from_image_bytes(
        "firefox_icon",
        include_bytes!("../assets/firefox_icon.png"),
    )
    .unwrap();

    let gmail_icon =
        RetainedImage::from_image_bytes("gmail_icon", include_bytes!("../assets/gmail_icon.png"))
            .unwrap();

    let libre_office_icon = RetainedImage::from_image_bytes(
        "libre_office_icon",
        include_bytes!("../assets/libre_office_icon.png"),
    )
    .unwrap();

    let icons = vec![firefox_icon, gmail_icon, libre_office_icon];

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "OOBE",
        native_options,
        Box::new(|_| Box::new(OobeApp::new(icons))),
    )
    .unwrap();
}

#[derive(Default)]
struct OobeApp {
    icons: Vec<RetainedImage>,
}

impl OobeApp {
    /// Called once before the first frame.
    pub fn new(icons: Vec<RetainedImage>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self { icons }
    }
}

impl eframe::App for OobeApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { icons } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OOBE");

            ui.horizontal(|ui| {
                icons[0].show_scaled(ui, 0.3);
                icons[1].show_scaled(ui, 0.06);
                icons[2].show_scaled(ui, 0.3);
            })
        });
    }
}
