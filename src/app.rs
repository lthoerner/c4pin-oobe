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

enum EntryFieldType {
    Fullname,
    Username,
    Password,
    ConfirmPassword,
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

    fn add_heading(&mut self, ui: &mut Ui, text: &str, size: f32, margin: f32) {
        ui.add_space(margin);
        let text = rich(text, size, FontType::Bold).color(hex_color!("#282828"));
        ui.heading(text);
    }

    fn add_button(&mut self, ui: &mut Ui, ctx: &egui::Context, button_type: ButtonType) {
        use ButtonType::*;
        let button = ImageButton::new(
            match button_type {
                Start => self.start_button_image.texture_id(ctx),
                Next => self.next_button_image.texture_id(ctx),
                Finish => self.finish_button_image.texture_id(ctx),
            },
            Vec2::new(335.0, 96.0),
        )
        .frame(false);

        let bottom_alignment = Layout::bottom_up(Align::Center);
        ui.with_layout(bottom_alignment, |ui| {
            ui.add_space(62.0);

            if ui.add(button).clicked() {
                self.current_page.advance()
            }
        });
    }

    fn add_optional_program(&mut self, ui: &mut Ui, name: &str, description: &str, icon: Icon) {
        ui.horizontal(|ui| {
            use Icon::*;
            match icon {
                Zoom => &self.zoom_icon,
                Vlc => &self.vlc_icon,
                LoWriter => &self.lo_writer_icon,
                LoCalc => &self.lo_calc_icon,
                LoImpress => &self.lo_impress_icon,
            }
            .show_scaled(ui, 0.25);

            ui.vertical(|ui| {
                self.add_heading(ui, name, 39.0, 0.0);
                ui.label(rich(description, 29.0, FontType::Regular));
            });
        });
    }

    fn add_entry_field(
        &mut self,
        ui: &mut Ui,
        name: &str,
        hint: Option<&str>,
        entry_type: EntryFieldType,
        password: bool,
    ) {
        ui.style_mut().visuals.extreme_bg_color = Color32::LIGHT_GRAY;
        ui.label(rich(name, 39.0, FontType::Bold));

        let edit_text = match entry_type {
            EntryFieldType::Fullname => &mut self.account_info.name,
            EntryFieldType::Username => &mut self.account_info.username,
            EntryFieldType::Password => &mut self.account_info.password,
            EntryFieldType::ConfirmPassword => &mut self.account_info.confirm_password,
        };

        ui.add(
            TextEdit::singleline(edit_text)
                .min_size(Vec2::new(440.0, 54.0))
                .font(FontId::new(35.0, FontType::Medium.into()))
                .password(password)
                .hint_text(RichText::new(hint.unwrap_or_default()).color(hex_color!("#737373"))),
        );
    }

    fn render_start_page(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            self.add_heading(ui, "Let's get you\nstarted.", 142.0, 170.0);
        });

        self.add_button(ui, ctx, ButtonType::Start);
    }

    fn render_firefox_page(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            self.add_heading(ui, "You can use Firefox to\nbrowse the web.", 101.0, 104.0);
            ui.add_space(15.0);
            self.firefox_icon.show_scaled(ui, 0.25);
        });

        self.add_button(ui, ctx, ButtonType::Next);
    }

    fn render_gmail_page(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            self.add_heading(
                ui,
                "You can use Gmail to\nsend and receive emails.",
                101.0,
                104.0,
            );

            ui.add_space(35.0);
            self.gmail_icon.show_scaled(ui, 0.25);
        });

        self.add_button(ui, ctx, ButtonType::Next);
    }

    fn render_optionals_page(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        inner_frame: &egui::Frame,
    ) {
        ui.vertical_centered(|ui| {
            self.add_heading(ui, "Select optional programs.", 101.0, 104.0);

            ui.add_space(38.0);

            ui.allocate_ui(Vec2::new(1263.0, 500.0), |ui| {
                inner_frame.show(ui, |ui| {
                    let scroll_area = ScrollArea::vertical().auto_shrink([false, false]);
                    scroll_area.show(ui, |ui| {
                        ui.vertical(|ui| {
                            self.add_optional_program(
                                ui,
                                "Zoom",
                                "Join video calls with friends, family, and coworkers.",
                                Icon::Zoom,
                            );

                            ui.separator();

                            self.add_optional_program(
                                ui,
                                "VLC",
                                "Play audio and video files, such as music and movies.",
                                Icon::Vlc,
                            );

                            ui.separator();

                            self.add_optional_program(
                                ui,
                                "LibreOffice Writer",
                                "Create and edit document, similar to MS Word.",
                                Icon::LoWriter,
                            );

                            ui.separator();

                            self.add_optional_program(
                                ui,
                                "LibreOffice Calc",
                                "Create and edit spreadsheets, similar to MS Excel.",
                                Icon::LoCalc,
                            );

                            ui.separator();

                            self.add_optional_program(
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

        self.add_button(ui, ctx, ButtonType::Next);
    }

    fn render_account_page(&mut self, ui: &mut Ui, ctx: &egui::Context, inner_frame: &egui::Frame) {
        ui.vertical_centered(|ui| {
            self.add_heading(ui, "Create a user account.", 101.0, 104.0);

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
                                        use EntryFieldType::*;
                                        strip.cell(|ui| {
                                            let left_layout = Layout::top_down(Align::Min);
                                            ui.with_layout(left_layout, |ui| {
                                                self.add_entry_field(
                                                    ui,
                                                    "Full Name",
                                                    Some("Willem Dafoe"),
                                                    Fullname,
                                                    false,
                                                );
                                                self.add_entry_field(
                                                    ui,
                                                    "Username",
                                                    Some("willdafoe"),
                                                    Username,
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
                                                self.add_entry_field(ui, "Password", None, Password, true);
                                                self.add_entry_field(ui, "Confirm Password", None, ConfirmPassword, true);
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

        self.add_button(ui, ctx, ButtonType::Finish);
    }
}

impl eframe::App for OobeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Outer frame is used for the background pattern and the main UI.
        let outer_frame = egui::Frame {
            fill: Color32::TRANSPARENT,
            inner_margin: Margin::same(0.0),
            ..Default::default()
        };

        // Inner frame is used for the optional programs list and account creation box.
        let inner_frame = egui::Frame {
            inner_margin: Margin::symmetric(38.0, 38.0),
            rounding: Rounding::default().at_least(28.0),
            fill: Color32::WHITE,
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
                Start => self.render_start_page(ui, ctx),
                Firefox => self.render_firefox_page(ui, ctx),
                Gmail => self.render_gmail_page(ui, ctx),
                Optionals => self.render_optionals_page(ui, ctx, &inner_frame),
                Account => self.render_account_page(ui, ctx, &inner_frame),
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
