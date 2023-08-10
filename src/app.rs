use eframe::CreationContext;
use egui::{
    hex_color, Align, CentralPanel, Color32, Direction, FontData, FontDefinitions, FontFamily,
    FontId, ImageButton, Layout, Margin, RichText, Rounding, ScrollArea, TextEdit, Ui, Vec2,
};
use egui_extras::{RetainedImage, Size, StripBuilder};

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

impl OobeApp {
    pub fn new(context: &CreationContext) -> Self {
        let mut fonts = FontDefinitions::default();
        let families = &mut fonts.families;

        macro_rules! add_font {
            ($name:literal) => {
                fonts.font_data.insert(
                    $name.to_owned(),
                    FontData::from_static(include_bytes!(concat!("../assets/", $name, ".otf"))),
                );

                families.insert(FontFamily::Name($name.into()), vec![$name.to_owned()]);
            };
        }

        add_font!("sf_pro_bold");
        add_font!("sf_pro_medium");
        add_font!("sf_pro_regular");

        context.egui_ctx.set_fonts(fonts);

        macro_rules! get_image {
            ($name:literal) => {
                RetainedImage::from_image_bytes(
                    $name,
                    include_bytes!(concat!("../assets/", $name, ".png")),
                )
                .unwrap()
            };
        }

        Self {
            current_page: Page::default(),
            optional_programs: OptionalPrograms::default(),
            account_info: AccountInfo::default(),
            background_image: get_image!("polkadot_background"),
            start_button_image: get_image!("start_button"),
            next_button_image: get_image!("next_button"),
            finish_button_image: get_image!("finish_button"),
            firefox_icon: get_image!("firefox_icon"),
            gmail_icon: get_image!("gmail_icon"),
            zoom_icon: get_image!("zoom_icon"),
            vlc_icon: get_image!("vlc_icon"),
            lo_writer_icon: get_image!("lo_writer_icon"),
            lo_calc_icon: get_image!("lo_calc_icon"),
            lo_impress_icon: get_image!("lo_impress_icon"),
        }
    }
}

impl eframe::App for OobeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let add_heading = |ui: &mut Ui, text: &str, margin: f32, size: f32| {
            ui.add_space(margin);

            let text = rich(text, size, FontType::Bold).color(hex_color!("#282828"));
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
                        ui.label(rich(description, 29.0, FontType::Regular));
                    });
                });
            };

        let add_entry_field =
            |ui: &mut Ui, name: &str, hint: Option<&str>, editing: &mut String, password: bool| {
                ui.style_mut().visuals.extreme_bg_color = Color32::LIGHT_GRAY;
                ui.label(rich(name, 39.0, FontType::Bold));
                ui.add(
                    TextEdit::singleline(editing)
                        .min_size(Vec2::new(440.0, 54.0))
                        .font(FontId::new(35.0, FontType::Medium.into()))
                        .password(password)
                        .hint_text(
                            RichText::new(hint.unwrap_or_default()).color(hex_color!("#737373")),
                        ),
                );
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
                        add_heading(
                            ui,
                            "You can use Gmail to\nsend and receive emails.",
                            104.0,
                            101.0,
                        );

                        ui.add_space(35.0);

                        self.gmail_icon.show_scaled(ui, 0.25);
                    });

                    add_button(self, ui, ButtonType::Next);
                }
                Optionals => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "Select optional programs.", 104.0, 101.0);

                        ui.add_space(38.0);

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

                        ui.add_space(38.0);

                        StripBuilder::new(ui).size(Size::exact(500.0)).vertical(|mut strip| {
                            strip.cell(|ui| {
                                StripBuilder::new(ui).size(Size::remainder()).size(Size::exact(1263.0)).size(Size::remainder()).horizontal(|mut strip| {
                                    strip.empty();
                                    strip.cell(|ui| {
                                        inner_frame
                                        .show(ui, |ui| {
                                            StripBuilder::new(ui)
                                                .size(Size::exact(440.0))
                                                .size(Size::remainder())
                                                .size(Size::exact(440.0))
                                                .horizontal(|mut strip| {
                                                    strip.cell(|ui| {
                                                        let left_layout = Layout::top_down(Align::Min);
                                                        ui.with_layout(left_layout, |ui| {
                                                            add_entry_field(
                                                                ui,
                                                                "Full Name",
                                                                Some("Willem Dafoe"),
                                                                &mut self.account_info.name,
                                                                false,
                                                            );
                                                            add_entry_field(
                                                                ui,
                                                                "Username",
                                                                Some("willdafoe"),
                                                                &mut self.account_info.username,
                                                                false,
                                                            );
                                                        });
                                                    });
        
                                                    strip.cell(|ui| {
                                                        ui.with_layout(
                                                            Layout::centered_and_justified(
                                                                Direction::RightToLeft,
                                                            ),
                                                            |ui| {
                                                                ui.separator();
                                                            },
                                                        );
                                                    });
        
                                                    strip.cell(|ui| {
                                                        let right_layout = Layout::top_down(Align::Max);
                                                        ui.with_layout(right_layout, |ui| {
                                                            add_entry_field(ui, "Password", None, &mut self.account_info.password, true);
                                                            add_entry_field(ui, "Confirm Password", None, &mut self.account_info.confirm_password, true);
                                                            ui.label(rich("If you forget this password, you will\nlose all of your files and programs.", 24.0, FontType::Medium));
                                                        });
                                                    });
                                            });
                                        });
                                    });
        
                                    strip.empty();
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

enum FontType {
    Regular,
    Medium,
    Bold,
}

impl From<FontType> for FontFamily {
    fn from(value: FontType) -> Self {
        match value {
            FontType::Regular => FontFamily::Name("sf_pro_regular".into()),
            FontType::Medium => FontFamily::Name("sf_pro_medium".into()),
            FontType::Bold => FontFamily::Name("sf_pro_bold".into()),
        }
    }
}

fn rich(text: &str, size: f32, font_type: FontType) -> RichText {
    let family = FontFamily::Name(match font_type {
        FontType::Regular => "sf_pro_regular".into(),
        FontType::Medium => "sf_pro_medium".into(),
        FontType::Bold => "sf_pro_bold".into(),
    });

    RichText::new(text).font(FontId::new(size, family))
}
