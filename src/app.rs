use eframe::CreationContext;
use egui::{
    hex_color, Align, CentralPanel, Color32, FontData, FontDefinitions, FontFamily, FontId,
    ImageButton, Layout, Margin, RichText, Rounding, ScrollArea, TextEdit, Ui, Vec2,
};
use egui_extras::RetainedImage;

pub struct OobeApp {
    current_page: Page,
    optional_programs: OptionalPrograms,
    account_info: AccountInfo,
    background_image: RetainedImage,
    start_button_image: RetainedImage,
    next_button_image: RetainedImage,
    finish_button_image: RetainedImage,
    firefox_icon: RetainedImage,
    gmail_icon: RetainedImage,
    zoom_icon: RetainedImage,
    vlc_icon: RetainedImage,
    lo_writer_icon: RetainedImage,
    lo_calc_icon: RetainedImage,
    lo_impress_icon: RetainedImage,
}

impl OobeApp {
    pub fn new(context: &CreationContext) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "sf_pro_bold".to_owned(),
            FontData::from_static(include_bytes!("../assets/SF-Pro-Display-Bold.otf")),
        );
        // fonts.font_data.insert(
        //     "sf_pro_medium".to_owned(),
        //     FontData::from_static(include_bytes!("../assets/SF-Pro-Display-Medium.otf")),
        // );
        // fonts.font_data.insert(
        //     "sf_pro_regular".to_owned(),
        //     FontData::from_static(include_bytes!("../assets/SF-Pro-Display-Regular.otf")),
        // );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "sf_pro_bold".to_owned());

        context.egui_ctx.set_fonts(fonts);

        Self {
            current_page: Page::default(),
            optional_programs: OptionalPrograms::default(),
            account_info: AccountInfo::default(),
            background_image: RetainedImage::from_image_bytes(
                "polkadot_background",
                include_bytes!("../assets/polkadot_background.png"),
            )
            .unwrap(),
            start_button_image: RetainedImage::from_image_bytes(
                "start_button",
                include_bytes!("../assets/start_button.png"),
            )
            .unwrap(),
            next_button_image: RetainedImage::from_image_bytes(
                "next_button",
                include_bytes!("../assets/next_button.png"),
            )
            .unwrap(),
            finish_button_image: RetainedImage::from_image_bytes(
                "finish_button",
                include_bytes!("../assets/finish_button.png"),
            )
            .unwrap(),
            firefox_icon: RetainedImage::from_image_bytes(
                "firefox_icon",
                include_bytes!("../assets/firefox_icon.png"),
            )
            .unwrap(),
            gmail_icon: RetainedImage::from_image_bytes(
                "gmail_icon",
                include_bytes!("../assets/gmail_icon.png"),
            )
            .unwrap(),
            zoom_icon: RetainedImage::from_image_bytes(
                "zoom_icon",
                include_bytes!("../assets/zoom_icon.png"),
            )
            .unwrap(),
            vlc_icon: RetainedImage::from_image_bytes(
                "vlc_icon",
                include_bytes!("../assets/vlc_icon.png"),
            )
            .unwrap(),
            lo_writer_icon: RetainedImage::from_image_bytes(
                "lo_writer_icon",
                include_bytes!("../assets/lo_writer_icon.png"),
            )
            .unwrap(),
            lo_calc_icon: RetainedImage::from_image_bytes(
                "lo_calc_icon",
                include_bytes!("../assets/lo_writer_icon.png"),
            )
            .unwrap(),
            lo_impress_icon: RetainedImage::from_image_bytes(
                "lo_impress_icon",
                include_bytes!("../assets/lo_writer_icon.png"),
            )
            .unwrap(),
        }
    }
}

#[derive(Default, Clone, Copy)]
enum Page {
    #[default]
    Start,
    Firefox,
    Gmail,
    Optionals,
    Account,
}

impl Page {
    // Advances the page when the user clicks the appropriate button.
    fn advance(&mut self) {
        use Page::*;
        *self = match self {
            Start => Firefox,
            Firefox => Gmail,
            Gmail => Optionals,
            Optionals => Account,
            // TODO: This will be replaced later with a function or method.
            Account => std::process::exit(0),
        }
    }
}

#[derive(Default)]
struct OptionalPrograms {
    zoom: bool,
    vlc: bool,
    lo_writer: bool,
    lo_calc: bool,
    lo_impress: bool,
}

#[derive(Default)]
struct AccountInfo {
    name: String,
    username: String,
    password: String,
    confirm_password: String,
}

enum ButtonType {
    Start,
    Next,
    Finish,
}

enum Icon {
    Zoom,
    Vlc,
    LoWriter,
    LoCalc,
    LoImpress,
}

impl eframe::App for OobeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let add_heading = |ui: &mut Ui, text: &str, margin: f32, size: f32| {
            ui.add_space(margin);

            let text = rich(text, size).color(hex_color!("#282828"));
            ui.heading(text);
        };

        let add_button = |app: &mut OobeApp, ui: &mut Ui, button_type: ButtonType| {
            use ButtonType::*;
            let button = ImageButton::new(
                match button_type {
                    Start => app.start_button_image.texture_id(ctx),
                    Next => app.next_button_image.texture_id(ctx),
                    Finish => app.finish_button_image.texture_id(ctx),
                },
                Vec2::new(335.0, 96.0),
            )
            .frame(false);

            let bottom_alignment = Layout::bottom_up(Align::Center);
            ui.with_layout(bottom_alignment, |ui| {
                ui.add_space(62.0);

                if ui.add(button).clicked() {
                    app.current_page.advance()
                }
            });
        };

        let add_optional_program =
            |app: &mut OobeApp, ui: &mut Ui, name: &str, description: &str, icon: Icon| {
                ui.horizontal(|ui| {
                    use Icon::*;
                    match icon {
                        Zoom => &app.zoom_icon,
                        Vlc => &app.vlc_icon,
                        LoWriter => &app.lo_writer_icon,
                        LoCalc => &app.lo_calc_icon,
                        LoImpress => &app.lo_impress_icon,
                    }
                    .show_scaled(ui, 0.25);

                    ui.vertical(|ui| {
                        add_heading(ui, name, 0.0, 39.0);
                        ui.label(rich(description, 29.0));
                    });
                });
            };

        let add_entry_field = |ui: &mut Ui, name: &str, editing: &mut String| {
            ui.style_mut().visuals.extreme_bg_color = Color32::LIGHT_GRAY;
            ui.label(rich(name, 17.0));
            ui.add(TextEdit::singleline(editing).desired_width(200.0));
        };

        // Inner frame for the optional programs list and account creation box.
        let inner_frame = egui::Frame {
            inner_margin: Margin::symmetric(38.0, 38.0),
            rounding: Rounding::default().at_least(28.0),
            fill: Color32::WHITE,
            ..Default::default()
        };

        let outer_frame = egui::Frame {
            fill: Color32::TRANSPARENT,
            inner_margin: Margin::same(0.0),
            ..Default::default()
        };

        // Add the background pattern to render the main UI over.
        CentralPanel::default().frame(outer_frame).show(ctx, |ui| {
            ui.image(
                self.background_image.texture_id(ctx),
                Vec2::new(1512.0, 982.0),
            );
        });

        CentralPanel::default().frame(outer_frame).show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(Color32::BLACK);

            use Page::*;
            match self.current_page {
                Start => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "Let's get you started.", 142.0, 170.0);
                    });

                    add_button(self, ui, ButtonType::Start);
                }
                Firefox => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "You can use Firefox to\nbrowse the web.", 104.0, 101.0);

                        ui.add_space(15.0);

                        self.firefox_icon.show_scaled(ui, 0.25);
                    });

                    add_button(self, ui, ButtonType::Next);
                }
                Gmail => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "You can use Gmail to\nsend and receive emails.", 104.0, 101.0);

                        ui.add_space(35.0);

                        self.gmail_icon.show_scaled(ui, 0.25);
                    });

                    add_button(self, ui, ButtonType::Next);
                }
                Optionals => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "Select optional programs.", 104.0, 101.0);

                        ui.add_space(15.0);

                        ui.allocate_ui(Vec2::new(1263.0, 500.0), |ui| {
                            inner_frame.show(ui, |ui| {
                                let scroll_area =
                                    ScrollArea::vertical().auto_shrink([false, false]);
                                scroll_area.show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        add_optional_program(
                                            self,
                                            ui,
                                            "Zoom",
                                            "Join video calls with friends, family, and coworkers.",
                                            Icon::Zoom,
                                        );

                                        ui.separator();

                                        add_optional_program(
                                            self,
                                            ui,
                                            "VLC",
                                            "Play audio and video files, such as music and movies.",
                                            Icon::Vlc,
                                        );

                                        ui.separator();

                                        add_optional_program(
                                            self,
                                            ui,
                                            "LibreOffice Writer",
                                            "Create and edit document, similar to MS Word.",
                                            Icon::LoWriter,
                                        );

                                        ui.separator();

                                        add_optional_program(
                                            self,
                                            ui,
                                            "LibreOffice Calc",
                                            "Create and edit spreadsheets, similar to MS Excel.",
                                            Icon::LoCalc,
                                        );

                                        ui.separator();

                                        add_optional_program(
                                            self,
                                            ui,
                                            "LibreOffice Impress",
                                            "Create and edit slideshows, similar to MS PowerPoint.",
                                            Icon::LoImpress,
                                        );
                                    });
                                });
                            });
                        });
                    });

                    add_button(self, ui, ButtonType::Next);
                }
                Account => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "Create a user account.", 104.0, 101.0);

                        ui.allocate_ui(Vec2::new(1263.0, 500.0), |ui| {
                            inner_frame.show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        add_entry_field(ui, "Full Name", &mut self.account_info.name);
                                        add_entry_field(ui, "Username", &mut self.account_info.username);
                                    });

                                    ui.add_space(36.0);
                                    ui.separator();
                                    ui.add_space(36.0);

                                    ui.vertical(|ui| {
                                        add_entry_field(ui, "Password", &mut self.account_info.password);
                                        add_entry_field(ui, "Confirm Password", &mut self.account_info.confirm_password);
                                        ui.label(rich("If you forget this password, you will lose all of your files and programs.", 12.0));
                                    });
                                });
                            });
                        });
                    });

                    add_button(self, ui, ButtonType::Finish);
                }
            }
        });
    }
}

fn rich(text: &str, size: f32) -> RichText {
    RichText::new(text).font(FontId::proportional(size))
}
