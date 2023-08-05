use egui::{
    hex_color, Button, CentralPanel, Color32, FontData, FontDefinitions, FontFamily, FontId,
    RichText, Rounding, Vec2,
};
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

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1512.0, 982.0)),
        ..Default::default()
    };

    eframe::run_native(
        "C4PIN OOBE",
        native_options,
        Box::new(|ctx| Box::new(OobeApp::new(ctx, icons))),
    )
    .unwrap();
}

#[derive(Default)]
struct OobeApp {
    icons: Vec<RetainedImage>,
    current_page: Page,
}

#[derive(Default)]
enum Page {
    #[default]
    Start,
    Firefox,
    Gmail,
    OptonalPrograms,
}

impl OobeApp {
    /// Called once before the first frame.
    pub fn new(ctx: &eframe::CreationContext, icons: Vec<RetainedImage>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "sf_pro_semibold".to_owned(),
            FontData::from_static(include_bytes!("../assets/sf_pro_semibold.otf")),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "sf_pro_semibold".to_owned());

        ctx.egui_ctx.set_fonts(fonts);

        Self {
            icons,
            current_page: Page::Start,
        }
    }
}

impl eframe::App for OobeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::Frame {
            fill: hex_color!("#DBFFF6"),
            ..Default::default()
        };

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(142.0);

                let heading = RichText::new("Let's get you started.")
                    .font(FontId::proportional(170.0))
                    .color(Color32::BLACK);
                ui.heading(heading);

                ui.add_space(91.0);

                let start_button_text = RichText::new("Start")
                    .font(FontId::proportional(63.0))
                    .color(Color32::WHITE);
                let start_button = Button::new(start_button_text)
                    .min_size(Vec2::new(413.0, 138.0))
                    .fill(hex_color!("#3D00A1"))
                    .rounding(Rounding::default().at_least(17.0));

                if ui.add(start_button).clicked() {
                    todo!()
                }
            })
        });
    }
}
