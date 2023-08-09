use egui::{
    Align, Button, CentralPanel, Color32, FontId, Layout, Margin, RichText, Rounding, ScrollArea,
    TextEdit, Ui, Vec2,
};
use egui_extras::RetainedImage;

#[derive(Default)]
pub struct OobeApp {
    current_page: Page,
    optional_programs: OptionalPrograms,
    account_info: AccountInfo,
    background_image: Option<RetainedImage>,
}

impl OobeApp {
    pub fn new(background_image: RetainedImage) -> Self {
        Self {
            background_image: Some(background_image),
            ..Default::default()
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

impl eframe::App for OobeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let add_heading = |ui: &mut Ui, text: &str, margin: f32, size: f32| {
            ui.add_space(margin);
            ui.heading(rich(text, size));
        };

        let add_button = |app: &mut OobeApp, ui: &mut Ui, text: &str| {
            let button_text = RichText::new(text)
                .font(FontId::proportional(38.0))
                .color(Color32::WHITE);
            let button = Button::new(button_text)
                .min_size(Vec2::new(335.0, 96.0))
                .rounding(Rounding::default().at_least(17.0));

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
            if let Some(background_image) = &self.background_image {
                ui.image(background_image.texture_id(ctx), Vec2::new(1512.0, 982.0));
            }
        });

        CentralPanel::default().frame(outer_frame).show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(Color32::BLACK);

            use Page::*;
            match self.current_page {
                Start => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "Let's get you started.", 142.0, 170.0);
                    });

                    add_button(self, ui, "Start");
                }
                Firefox => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "You can use Firefox to browse the web.", 104.0, 101.0);
                    });

                    add_button(self, ui, "Next");
                }
                Gmail => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "You can use Gmail to send and receive emails.", 104.0, 101.0);
                    });

                    add_button(self, ui, "Next");
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

                    add_button(self, ui, "Next");
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

                    add_button(self, ui, "Finish");
                }
            }
        });
    }
}

fn rich(text: &str, size: f32) -> RichText {
    RichText::new(text).font(FontId::proportional(size))
}
