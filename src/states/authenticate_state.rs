#[derive(Default, Debug)]
pub struct AuthenticateState {
    pub error_message: Option<String>,
    pub password: String,
}

impl AuthenticateState {
    pub fn render(&mut self, ctx: &egui::Context) -> bool {
        let mut result = false;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("Please enter your password:");
                ui.group(|ui| {
                    ui.spacing_mut().item_spacing = egui::Vec2::new(2.0, 10.0);

                    ui.add(egui::TextEdit::singleline(&mut self.password).password(true));

                    if let Some(auth_err) = &self.error_message {
                        ui.add(egui::Label::new(auth_err));
                    }

                    if ui.button("Authenticate").clicked() {
                        result = true;
                    }
                });
            })
        });

        result
    }
}
