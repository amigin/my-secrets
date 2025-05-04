use std::collections::BTreeMap;

use encryption::aes::AesKey;

use crate::{states::*, MyApp};

pub enum DialogResult {
    Ok,
    Cancel,
}

pub enum ShowDialogResult {
    DialogIsBeingRendered,
    CreatedCategory(String),
    RenameCategory(String),
    CreatedSubCategory(String),
    Authenticated {
        aes_key: AesKey,
        data: BTreeMap<String, BTreeMap<String, String>>,
    },
    Cancel,
}

impl MyApp {
    pub fn render_dialog(&mut self, ctx: &egui::Context) -> Option<ShowDialogResult> {
        let modal_dialog = self.modal_dialog.get_mut();

        //println!("{:?}", modal_dialog);

        match modal_dialog {
            ModalWindowState::None => {
                if self.edit_state.activity_expired() {
                    self.authenticated = None;
                    self.modal_dialog
                        .set(ModalWindowState::Authenticate(Default::default()));
                }

                return None;
            }
            ModalWindowState::Authenticate(state) => {
                if state.render(ctx) {
                    let password =
                        crate::password_utils::make_password_complient(state.password.as_bytes());

                    let aes_key = AesKey {
                        key: password,
                        iv: self.settings.get_iv(),
                    };

                    if let Some(data) = crate::file::load_file(&aes_key) {
                        return Some(ShowDialogResult::Authenticated { aes_key, data });
                    } else {
                        state.error_message = "Invalid password".to_string().into();
                        return Some(ShowDialogResult::DialogIsBeingRendered);
                    }
                }
                return Some(ShowDialogResult::DialogIsBeingRendered);
            }
            ModalWindowState::CreateCategory(category) => {
                if let Some(dialog_result) =
                    render_edit_modal(ctx, "Enter category name:", "Add", category)
                {
                    match dialog_result {
                        DialogResult::Ok => {
                            self.authenticated
                                .as_mut()
                                .unwrap()
                                .content
                                .insert(category.to_string(), BTreeMap::new());
                            self.has_not_saved_data = true;
                            return Some(ShowDialogResult::CreatedCategory(category.to_string()));
                        }

                        DialogResult::Cancel => {
                            return Some(ShowDialogResult::Cancel);
                        }
                    }
                }
                return Some(ShowDialogResult::DialogIsBeingRendered);
            }
            ModalWindowState::RenameCategory(category) => {
                if let Some(dialog_result) =
                    render_edit_modal(ctx, "Enter category name:", "Rename", category)
                {
                    match dialog_result {
                        DialogResult::Ok => {
                            return Some(ShowDialogResult::RenameCategory(category.to_string()));
                        }
                        DialogResult::Cancel => {
                            return Some(ShowDialogResult::Cancel);
                        }
                    }
                }
                return Some(ShowDialogResult::DialogIsBeingRendered);
            }

            ModalWindowState::CreateSubCategory(sub_category) => {
                if let Some(dialog_result) =
                    render_edit_modal(ctx, "Enter subcategory name:", "Add", sub_category)
                {
                    match dialog_result {
                        DialogResult::Ok => {
                            self.authenticated
                                .as_mut()
                                .unwrap()
                                .content
                                .get_mut(self.selected_category.as_ref().unwrap())
                                .unwrap()
                                .insert(sub_category.to_string(), "".to_string());

                            self.has_not_saved_data = true;
                            return Some(ShowDialogResult::CreatedSubCategory(
                                sub_category.to_string(),
                            ));
                        }

                        DialogResult::Cancel => {
                            return Some(ShowDialogResult::Cancel);
                        }
                    }
                }
                return Some(ShowDialogResult::DialogIsBeingRendered);
            }
        }
    }
}

fn render_edit_modal(
    ctx: &egui::Context,
    title: &str,
    ok_btn: &str,

    value: &mut String,
) -> Option<DialogResult> {
    let mut result = None;
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.set_width(400.0);

            ui.heading(title);
            ui.group(|ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::new(2.0, 10.0);

                ui.vertical_centered_justified(|ui| {
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(value));
                        if ui.button(ok_btn).clicked() {
                            result = Some(DialogResult::Ok);
                        }

                        if ui.button("Cancel").clicked() {
                            result = Some(DialogResult::Cancel);
                        }
                    });
                });
            });
        })
    });

    result
}
