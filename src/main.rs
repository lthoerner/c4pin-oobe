use egui::{
    hex_color, Align, Button, CentralPanel, Color32, FontData, FontDefinitions, FontFamily, FontId,
    Layout, Margin, RichText, Rounding, Vec2,
};
use egui_extras::RetainedImage;

fn main() {
    let firefox_icon = RetainedImage::from_image_bytes(
        "firefox_icon",
        include_bytes!("../assets/firefox_icon.png"),
    )
    .unwrap();

    let arrow_icon =
        RetainedImage::from_image_bytes("arrow_icon", include_bytes!("../assets/arrow_icon.png"))
            .unwrap();

    let globe_icon =
        RetainedImage::from_image_bytes("globe_icon", include_bytes!("../assets/globe_icon.png"))
            .unwrap();

    let gmail_icon =
        RetainedImage::from_image_bytes("gmail_icon", include_bytes!("../assets/gmail_icon.png"))
            .unwrap();

    let libre_office_icon = RetainedImage::from_image_bytes(
        "libre_office_icon",
        include_bytes!("../assets/libre_office_icon.png"),
    )
    .unwrap();

    let icons = vec![
        firefox_icon,
        arrow_icon,
        globe_icon,
        gmail_icon,
        libre_office_icon,
    ];

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

#[derive(Default, Clone, Copy)]
enum Page {
    #[default]
    Start,
    Firefox,
    Gmail,
    OptionalPrograms,
}

impl Page {
    // Advances the page when the user clicks the appropriate button.
    fn advance(&mut self) {
        use Page::*;
        *self = match self {
            Start => Firefox,
            Firefox => Gmail,
            Gmail => OptionalPrograms,
            OptionalPrograms => *self,
        }
    }
}

impl OobeApp {
    /// Called once before the first frame.
    pub fn new(ctx: &eframe::CreationContext, icons: Vec<RetainedImage>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "sf_pro_bold".to_owned(),
            FontData::from_static(include_bytes!("../assets/sf_pro_bold.otf")),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "sf_pro_bold".to_owned());

        ctx.egui_ctx.set_fonts(fonts);

        Self {
            icons,
            current_page: Page::Start,
        }
    }
}

impl eframe::App for OobeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let margin = Margin {
            left: 200.0,
            right: 200.0,
            ..Default::default()
        };

        let frame = egui::Frame {
            fill: hex_color!("#DBFFF6"),
            inner_margin: margin,
            ..Default::default()
        };

        let top_margin = 104f32;

        let next_button_text = RichText::new("Next")
            .font(FontId::proportional(38.0))
            .color(Color32::WHITE);
        let next_button = Button::new(next_button_text)
            .min_size(Vec2::new(335.0, 96.0))
            .fill(hex_color!("#3D00A1"))
            .rounding(Rounding::default().at_least(17.0));

        let bottom_alignment = Layout::bottom_up(Align::Center);

        CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| match self.current_page {
                Page::Start => {
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
                            self.current_page.advance()
                        }
                    });
                }
                Page::Firefox => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(top_margin);

                        ui.heading(heading_text("You can use Firefox to browse the web."));

                        ui.horizontal(|ui| {
                            ui.add_space(90.0);
                            self.icons[0].show_size(ui, Vec2::new(330.0, 330.0));
                            self.icons[1].show_size(ui, Vec2::new(204.0, 136.0));
                            self.icons[2].show_size(ui, Vec2::new(325.0, 324.0));
                        });

                        ui.with_layout(bottom_alignment, |ui| {
                            ui.add_space(64.0);

                            if ui.add(next_button).clicked() {
                                self.current_page.advance()
                            }
                        });
                    });
                }
                Page::Gmail => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(top_margin);

                        ui.heading(heading_text(
                            "You can use Gmail to send and receive emails.",
                        ));

                        self.icons[3].show_size(ui, Vec2::new(379.0, 284.0));

                        ui.with_layout(bottom_alignment, |ui| {
                            ui.add_space(64.0);

                            if ui.add(next_button).clicked() {
                                self.current_page.advance()
                            }
                        });
                    });
                }
                Page::OptionalPrograms => {
                    todo!()
                }
            });
    }
}

fn heading_text(text: &str) -> RichText {
    RichText::new(text)
        .font(FontId::proportional(101.0))
        .color(Color32::BLACK)
}
