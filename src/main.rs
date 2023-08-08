use egui::{
    Align, Button, CentralPanel, Color32, FontId, Layout, Margin, RichText, Rounding, ScrollArea,
    Ui, Vec2, TextEdit,
};

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(640.0, 360.0)),
        ..Default::default()
    };

    eframe::run_native(
        "OOBE",
        native_options,
        Box::new(|_| Box::<OobeApp>::default()),
    )
    .unwrap();
}

#[derive(Default)]
struct OobeApp {
    current_page: Page,
    optional_programs: OptionalPrograms,
    account_info: AccountInfo,
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
            Account => *self,
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
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let add_heading = |ui: &mut Ui, text: &str| {
            ui.add_space(20.0);
            ui.heading(rich(text, 48.0));
        };

        let add_button = |app: &mut OobeApp, ui: &mut Ui, text: &str| {
            let button_text = RichText::new(text).font(FontId::proportional(25.0));
            let button = Button::new(button_text)
                .min_size(Vec2::new(120.0, 40.0))
                .rounding(Rounding::default().at_least(17.0));

            let bottom_alignment = Layout::bottom_up(Align::Center);
            ui.with_layout(bottom_alignment, |ui| {
                ui.add_space(20.0);

                if ui.add(button).clicked() {
                    app.current_page.advance()
                }
            });
        };

        // Inner frame for the optional programs list and account creation box.
        let inner_frame = egui::Frame {
            inner_margin: Margin::symmetric(10.0, 10.0),
            rounding: Rounding::default().at_least(10.0),
            fill: Color32::GRAY,
            ..Default::default()
        };

        CentralPanel::default().show(ctx, |ui| {
            use Page::*;
            match self.current_page {
                Start => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "Let's get you started.");
                    });

                    add_button(self, ui, "Start");
                }
                Firefox => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "You can use Firefox to browse the web.");
                    });

                    add_button(self, ui, "Next");
                }
                Gmail => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "You can use Gmail to send and receive emails.");
                    });

                    add_button(self, ui, "Next");
                }
                Optionals => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "Select optional programs.");

                        ui.add_space(15.0);

                        ui.allocate_ui(Vec2::new(524.0, 170.0), |ui| {
                            ui.visuals_mut().override_text_color = Some(Color32::BLACK);
                            inner_frame.show(ui, |ui| {
                                let scroll_area =
                                    ScrollArea::vertical().auto_shrink([false, false]);
                                scroll_area.show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        ui.checkbox(
                                            &mut self.optional_programs.zoom,
                                            rich("Zoom", 30.0),
                                        );
                                        ui.label(rich(
                                            "Join video calls with friends, family, and coworkers.",
                                            20.0,
                                        ));

                                        ui.separator();

                                        ui.checkbox(
                                            &mut self.optional_programs.vlc,
                                            rich("VLC", 30.0),
                                        );
                                        ui.label(rich(
                                            "Play audio and video files, such as music and movies.",
                                            20.0,
                                        ));

                                        ui.separator();

                                        ui.checkbox(
                                            &mut self.optional_programs.libreoffice_writer,
                                            rich("LibreOffice Writer", 30.0),
                                        );
                                        ui.label(rich(
                                            "Create and edit document, similar to MS Word.",
                                            20.0,
                                        ));

                                        ui.separator();

                                        ui.checkbox(
                                            &mut self.optional_programs.libreoffice_calc,
                                            rich("LibreOffice Calc", 30.0),
                                        );
                                        ui.label(rich(
                                            "Create and edit spreadsheets, similar to MS Excel.",
                                            20.0,
                                        ));

                                        ui.separator();

                                        ui.checkbox(
                                            &mut self.optional_programs.libreoffice_impress,
                                            rich("LibreOffice Impress", 30.0),
                                        );
                                        ui.label(rich(
                                            "Create and edit slideshows, similar to MS PowerPoint.",
                                            20.0,
                                        ));
                                    });
                                });
                            });
                        });
                    });

                    add_button(self, ui, "Next");
                }
                Account => {
                    ui.vertical_centered(|ui| {
                        add_heading(ui, "Create a user account.");

                        ui.allocate_ui(Vec2::new(524.0, 170.0), |ui| {
                            ui.visuals_mut().override_text_color = Some(Color32::BLACK);
                            inner_frame.show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(rich("Full Name", 17.0));
                                        ui.add(TextEdit::singleline(&mut self.account_info.name).desired_width(200.0));
                                        ui.label(rich("Username", 17.0));
                                        ui.add(TextEdit::singleline(&mut self.account_info.username).desired_width(200.0));
                                    });

                                    ui.add_space(36.0);
                                    ui.separator();
                                    ui.add_space(36.0);

                                    ui.vertical(|ui| {
                                        ui.label(rich("Password", 17.0));
                                        ui.add(TextEdit::singleline(&mut self.account_info.password).desired_width(200.0));
                                        ui.label(rich("Confirm Password", 17.0));
                                        ui.add(TextEdit::singleline(&mut self.account_info.confirm_password).desired_width(200.0));
                                        
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
