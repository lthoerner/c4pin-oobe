use eframe::CreationContext;
use egui::{
    Align, CentralPanel, Color32, FontData, FontDefinitions, FontFamily, FontId, ImageButton,
    Layout, Margin, RichText, Rounding, ScrollArea, TextEdit, Ui, Vec2,
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
}

impl OobeApp {
    pub fn new(
        context: &CreationContext,
        background_image: RetainedImage,
        start_button_image: RetainedImage,
        next_button_image: RetainedImage,
        finish_button_image: RetainedImage,
    ) -> Self {
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
            background_image,
            start_button_image,
            next_button_image,
            finish_button_image,
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
    libreoffice_writer: bool,
    libreoffice_calc: bool,
    libreoffice_impress: bool,
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

impl eframe::App for OobeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let add_heading = |ui: &mut Ui, text: &str, margin: f32, size: f32| {
            ui.add_space(margin);
            ui.heading(rich(text, size));
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
            |ui: &mut Ui, name: &str, description: &str, editing: &mut bool| {
                ui.checkbox(editing, rich(name, 30.0));
                ui.label(rich(description, 20.0));
            };

        let add_entry_field = |ui: &mut Ui, name: &str, editing: &mut String| {
            ui.style_mut().visuals.extreme_bg_color = Color32::LIGHT_GRAY;
            ui.label(rich(name, 17.0));
            ui.add(TextEdit::singleline(editing).desired_width(200.0));
        };

        // Inner frame for the optional programs list and account creation box.
        let inner_frame = egui::Frame {
            inner_margin: Margin::symmetric(10.0, 10.0),
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
                        add_heading(ui, "You can use Firefox to browse the web.", 104.0, 101.0);
                    });

                    add_button(self, ui, ButtonType::Next);
                }
                Gmail => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "You can use Gmail to send and receive emails.", 104.0, 101.0);
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
                                            ui,
                                            "Zoom",
                                            "Join video calls with friends, family, and coworkers.",
                                            &mut self.optional_programs.zoom,
                                        );

                                        ui.separator();

                                        add_optional_program(
                                            ui,
                                            "VLC",
                                            "Play audio and video files, such as music and movies.",
                                            &mut self.optional_programs.vlc,
                                        );

                                        ui.separator();

                                        add_optional_program(
                                            ui,
                                            "LibreOffice Writer",
                                            "Create and edit document, similar to MS Word.",
                                            &mut self.optional_programs.libreoffice_writer,
                                        );

                                        ui.separator();

                                        add_optional_program(
                                            ui,
                                            "LibreOffice Calc",
                                            "Create and edit spreadsheets, similar to MS Excel.",
                                            &mut self.optional_programs.libreoffice_calc,
                                        );

                                        ui.separator();

                                        add_optional_program(
                                            ui,
                                            "LibreOffice Impress",
                                            "Create and edit slideshows, similar to MS PowerPoint.",
                                            &mut self.optional_programs.libreoffice_impress,
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
