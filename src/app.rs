use eframe::CreationContext;
use egui::{
    hex_color, scroll_area::ScrollBarVisibility, Align, CentralPanel, Color32, Direction, FontData,
    FontDefinitions, FontFamily, FontId, ImageButton, Layout, Margin, RichText, Rounding,
    ScrollArea, Separator, TextEdit, Ui, Vec2,
};
use egui_extras::{RetainedImage, Size, StripBuilder};

use crate::{bounds, centered_item, horizontal_strip, strip, vertical_strip};

pub struct OobeApp {
    current_page: Page,
    optional_program_state: OptionalPrograms,
    account_info_state: AccountInfo,
    button_states: ButtonStates,
    background_image: RetainedImage,
    start_button_image: RetainedImage,
    start_button_hovered_image: RetainedImage,
    next_button_image: RetainedImage,
    next_button_hovered_image: RetainedImage,
    finish_button_image: RetainedImage,
    finish_button_hovered_image: RetainedImage,
    firefox_icon: RetainedImage,
    gmail_icon: RetainedImage,
    zoom_icon: RetainedImage,
    vlc_icon: RetainedImage,
    lo_writer_icon: RetainedImage,
    lo_calc_icon: RetainedImage,
    lo_impress_icon: RetainedImage,
    checkbox_checked: RetainedImage,
    checkbox_unchecked: RetainedImage,
    checkbox_checked_outlined: RetainedImage,
    checkbox_unchecked_outlined: RetainedImage,
    warning_icon: RetainedImage,
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

struct OptionalPrograms {
    zoom: CheckboxState,
    vlc: CheckboxState,
    lo_writer: CheckboxState,
    lo_calc: CheckboxState,
    lo_impress: CheckboxState,
}

impl Default for OptionalPrograms {
    fn default() -> Self {
        Self {
            zoom: CheckboxState::checked(true),
            vlc: CheckboxState::checked(true),
            lo_writer: CheckboxState::checked(true),
            lo_calc: CheckboxState::checked(false),
            lo_impress: CheckboxState::checked(false),
        }
    }
}

struct CheckboxState {
    checked: bool,
    hovered: bool,
}

impl CheckboxState {
    fn checked(checked: bool) -> Self {
        Self {
            checked,
            hovered: false,
        }
    }
}

#[derive(Default)]
struct AccountInfo {
    name: String,
    username: String,
    password: String,
    confirm_password: String,
}

#[derive(Default)]
struct ButtonStates {
    start_page_button_hovered: bool,
    firefox_page_button_hovered: bool,
    gmail_page_button_hovered: bool,
    optionals_page_button_hovered: bool,
    account_page_button_hovered: bool,
}

enum OptionalProgram {
    Zoom,
    Vlc,
    LoWriter,
    LoCalc,
    LoImpress,
}

enum EntryField {
    Fullname,
    Username,
    Password,
    ConfirmPassword,
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
            optional_program_state: OptionalPrograms::default(),
            account_info_state: AccountInfo::default(),
            button_states: ButtonStates::default(),
            background_image: get_image!("polkadot_background"),
            start_button_image: get_image!("start_button"),
            start_button_hovered_image: get_image!("start_button_hovered"),
            next_button_image: get_image!("next_button"),
            next_button_hovered_image: get_image!("next_button_hovered"),
            finish_button_image: get_image!("finish_button"),
            finish_button_hovered_image: get_image!("finish_button_hovered"),
            firefox_icon: get_image!("firefox_icon"),
            gmail_icon: get_image!("gmail_icon"),
            zoom_icon: get_image!("zoom_icon"),
            vlc_icon: get_image!("vlc_icon"),
            lo_writer_icon: get_image!("lo_writer_icon"),
            lo_calc_icon: get_image!("lo_calc_icon"),
            lo_impress_icon: get_image!("lo_impress_icon"),
            checkbox_checked: get_image!("checkbox_checked"),
            checkbox_unchecked: get_image!("checkbox_unchecked"),
            checkbox_checked_outlined: get_image!("checkbox_checked_outlined"),
            checkbox_unchecked_outlined: get_image!("checkbox_unchecked_outlined"),
            warning_icon: get_image!("warning_icon"),
        }
    }

    fn add_heading(&mut self, ui: &mut Ui, text: &str, size: f32, margin: f32) {
        ui.add_space(margin);
        let text = rich(text, size, FontType::Bold).color(hex_color!("#282828"));
        ui.heading(text);
    }

    fn add_button(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let hovered = match self.current_page {
            Page::Start => &mut self.button_states.start_page_button_hovered,
            Page::Firefox => &mut self.button_states.firefox_page_button_hovered,
            Page::Gmail => &mut self.button_states.gmail_page_button_hovered,
            Page::Optionals => &mut self.button_states.optionals_page_button_hovered,
            Page::Account => &mut self.button_states.account_page_button_hovered,
        };

        let (standard_image, hovered_image) = match self.current_page {
            Page::Start => (&self.start_button_image, &self.start_button_hovered_image),
            Page::Firefox => (&self.next_button_image, &self.next_button_hovered_image),
            Page::Gmail => (&self.next_button_image, &self.next_button_hovered_image),
            Page::Optionals => (&self.next_button_image, &self.next_button_hovered_image),
            Page::Account => (&self.finish_button_image, &self.finish_button_hovered_image),
        };

        let (image, size) = match hovered {
            true => (hovered_image, Vec2::new(328.0, 94.0)),
            false => (standard_image, Vec2::new(335.0, 96.0)),
        };

        let button = ImageButton::new(image.texture_id(ctx), size).frame(false);

        let bottom_alignment = Layout::bottom_up(Align::Center);
        ui.with_layout(bottom_alignment, |ui| {
            ui.add_space(62.0);

            let button_listener = ui.add(button);
            *hovered = button_listener.hovered();
            if button_listener.clicked() {
                self.current_page.advance()
            }
        });
    }

    fn add_optional_program(&mut self, ui: &mut Ui, ctx: &egui::Context, program: OptionalProgram) {
        use OptionalProgram::*;
        let (program_icon, program_name, program_description, edit_state) = match program {
            Zoom => (
                &self.zoom_icon,
                "Zoom",
                "Join video calls with friends, family, and coworkers.",
                &mut self.optional_program_state.zoom,
            ),
            Vlc => (
                &self.vlc_icon,
                "VLC",
                "Play audio and video files, such as music and movies.",
                &mut self.optional_program_state.vlc,
            ),
            LoWriter => (
                &self.lo_writer_icon,
                "LibreOffice Writer",
                "Create and edit document, similar to MS Word.",
                &mut self.optional_program_state.lo_writer,
            ),
            LoCalc => (
                &self.lo_calc_icon,
                "LibreOffice Calc",
                "Create and edit spreadsheets, similar to MS Excel.",
                &mut self.optional_program_state.lo_calc,
            ),
            LoImpress => (
                &self.lo_impress_icon,
                "LibreOffice Impress",
                "Create and edit slideshows, similar to MS PowerPoint.",
                &mut self.optional_program_state.lo_impress,
            ),
        };

        vertical_strip!(ui, [120.0], |mut strip| {
            strip.cell(|ui| {
                // Strip containing an option program list item
                horizontal_strip!(ui, [111.0, 3.5, 700.0, auto, 108.0], |mut strip| {
                    // Cell for the program icon
                    strip.cell(|ui| {
                        ui.with_layout(
                            Layout::centered_and_justified(Direction::LeftToRight),
                            |ui| {
                                program_icon.show_scaled(ui, 0.25);
                            },
                        );
                    });
                    // Padding between the icon and the program name/description
                    strip.empty();
                    // Cell for the program name and description
                    strip.cell(|ui| {
                        ui.vertical(|ui| {
                            ui.add_space(10.0);
                            ui.label(rich(program_name, 39.0, FontType::Bold));
                            ui.label(rich(program_description, 29.0, FontType::Regular));
                        });
                    });
                    // Padding between the program name/description and the checkbox
                    strip.empty();
                    // Cell for the checkbox
                    strip.cell(|ui| {
                        centered_item!(ui, |ui| {
                            let button = ImageButton::new(
                                match (edit_state.checked, edit_state.hovered) {
                                    (true, false) => self.checkbox_checked.texture_id(ctx),
                                    (false, false) => self.checkbox_unchecked.texture_id(ctx),
                                    (true, true) => self.checkbox_checked_outlined.texture_id(ctx),
                                    (false, true) => {
                                        self.checkbox_unchecked_outlined.texture_id(ctx)
                                    }
                                },
                                Vec2::new(62.0, 62.0),
                            )
                            .frame(false);

                            let button_listener = ui.add(button);
                            edit_state.hovered = button_listener.hovered();
                            if button_listener.clicked() {
                                edit_state.checked = !edit_state.checked;
                            }
                        });
                    });
                });
            });
        });
    }

    fn add_entry_field(&mut self, ui: &mut Ui, entry_field: EntryField) {
        use EntryField::*;
        let (field_name, hint, hide_entry, edit_text) = match entry_field {
            Fullname => (
                "Full Name",
                Some("Willem Dafoe"),
                false,
                &mut self.account_info_state.name,
            ),
            Username => (
                "Username",
                Some("willdafoe"),
                false,
                &mut self.account_info_state.username,
            ),
            Password => (
                "Password",
                None,
                true,
                &mut self.account_info_state.password,
            ),
            ConfirmPassword => (
                "Confirm Password",
                None,
                true,
                &mut self.account_info_state.confirm_password,
            ),
        };

        // This isn't exactly the most elegant way to style the `TextEdit`s,
        // but it is the only way that I can figure out based on the docs.
        let visuals = ui.visuals_mut();
        visuals.extreme_bg_color = Color32::LIGHT_GRAY;
        visuals.widgets.active.rounding = Rounding::default().at_least(21.0);
        visuals.widgets.inactive.rounding = Rounding::default().at_least(21.0);
        visuals.widgets.hovered.rounding = Rounding::default().at_least(21.0);
        ui.label(rich(field_name, 39.0, FontType::Bold));
        ui.add(
            TextEdit::singleline(edit_text)
                .min_size(Vec2::new(440.0, 54.0))
                .font(FontId::new(35.0, FontType::Medium.into()))
                .password(hide_entry)
                .hint_text(RichText::new(hint.unwrap_or_default()).color(hex_color!("#737373")))
                .margin(Vec2::new(17.0, 7.0)),
        );
    }

    fn render_start_page(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            self.add_heading(ui, "Let's get you\nstarted.", 142.0, 170.0);
        });

        self.add_button(ui, ctx);
    }

    fn render_firefox_page(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            self.add_heading(ui, "You can use Firefox to\nbrowse the web.", 101.0, 104.0);
            ui.add_space(15.0);
            self.firefox_icon.show_scaled(ui, 0.25);
        });

        self.add_button(ui, ctx);
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

        self.add_button(ui, ctx);
    }

    fn render_optionals_page(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let frame = egui::Frame {
            inner_margin: Margin {
                left: 38.0,
                top: 30.0,
                right: 18.0,
                bottom: 30.0,
            },
            rounding: Rounding::default().at_least(28.0),
            fill: Color32::WHITE,
            ..Default::default()
        };

        ui.vertical_centered(|ui| {
            self.add_heading(ui, "Select optional programs.", 101.0, 104.0);

            ui.add_space(38.0);

            strip!(ui, 1263.0, 500.0, |mut strip| {
                strip.empty();
                // Cell containing the scrollable list
                strip.cell(|ui| {
                    frame.show(ui, |ui| {
                        // Add style to the scroll bar
                        let style = ui.style_mut();
                        style.visuals.widgets.active.bg_fill = hex_color!("#D9D9D9");
                        style.visuals.widgets.inactive.bg_fill = hex_color!("#D9D9D9");
                        style.visuals.widgets.hovered.bg_fill = hex_color!("#D9D9D9");
                        style.visuals.extreme_bg_color = Color32::TRANSPARENT;
                        style.visuals.widgets.active.rounding = Rounding::default().at_least(22.0);
                        style.visuals.widgets.inactive.rounding =
                            Rounding::default().at_least(22.0);
                        style.visuals.widgets.hovered.rounding = Rounding::default().at_least(22.0);
                        style.spacing.scroll_bar_width = 16.0;

                        let scroll_area = ScrollArea::vertical()
                            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible);
                        scroll_area.show(ui, |ui| {
                            // Sub-strip for inserting padding between the list items and the scroll bar
                            horizontal_strip!(ui, [auto, 18.0], |mut strip| {
                                strip.cell(|ui| {
                                    use OptionalProgram::*;
                                    self.add_optional_program(ui, ctx, Zoom);
                                    ui.add(Separator::default().spacing(10.0));
                                    self.add_optional_program(ui, ctx, Vlc);
                                    ui.add(Separator::default().spacing(10.0));
                                    self.add_optional_program(ui, ctx, LoWriter);
                                    ui.add(Separator::default().spacing(10.0));
                                    self.add_optional_program(ui, ctx, LoCalc);
                                    ui.add(Separator::default().spacing(10.0));
                                    self.add_optional_program(ui, ctx, LoImpress);
                                });
                                strip.empty();
                            });
                        });
                    });
                });
                strip.empty();
            });
        });

        self.add_button(ui, ctx);
    }

    fn render_account_page(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let frame = egui::Frame {
            inner_margin: Margin::symmetric(38.0, 38.0),
            rounding: Rounding::default().at_least(28.0),
            fill: Color32::WHITE,
            ..Default::default()
        };

        ui.vertical_centered(|ui| {
            self.add_heading(ui, "Create a user account.", 101.0, 104.0);

            ui.add_space(38.0);

            strip!(ui, 1263.0, 500.0, |mut strip| {
                strip.empty();
                strip.cell(|ui| {
                    frame.show(ui, |ui| {
                        horizontal_strip!(ui, [440.0, auto, 440.0], |mut strip| {
                            use EntryField::*;
                            strip.cell(|ui| {
                                let left_layout = Layout::top_down(Align::Min);
                                ui.with_layout(left_layout, |ui| {
                                    self.add_entry_field(ui, Fullname);
                                    self.add_entry_field(ui, Username);
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
                                    self.add_entry_field(ui, Password);
                                    self.add_entry_field(ui, ConfirmPassword);

                                    ui.add_space(24.0);

                                    horizontal_strip!(ui, [auto, 17.0, 358.0], |mut strip| {
                                        strip.empty();
                                        strip.cell(|ui| {
                                            ui.add_space(4.5);
                                            self.warning_icon.show_scaled(ui, 0.25);
                                        });
                                        strip.cell(|ui| {
                                            ui.label(rich("If you forget this password, you will\nlose all of your files and programs.", 24.0, FontType::Medium));
                                        });
                                    });
                                });

                            });
                        });
                    });
                });
                strip.empty();
            });
        });

        self.add_button(ui, ctx);
    }
}

impl eframe::App for OobeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::Frame {
            fill: Color32::TRANSPARENT,
            inner_margin: Margin::same(0.0),
            ..Default::default()
        };

        // Add the background pattern to render the main UI over
        CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.image(
                self.background_image.texture_id(ctx),
                Vec2::new(1512.0, 982.0),
            );
        });

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(Color32::BLACK);

            use Page::*;
            match self.current_page {
                Start => self.render_start_page(ui, ctx),
                Firefox => self.render_firefox_page(ui, ctx),
                Gmail => self.render_gmail_page(ui, ctx),
                Optionals => self.render_optionals_page(ui, ctx),
                Account => self.render_account_page(ui, ctx),
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
