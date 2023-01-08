use std::collections::BTreeMap;

use encryption::aes::AesKey;

use crate::MyApp;

pub enum ModalWindowState {
    Authenticate,
    CreateCategory(String),
    UpdateCategory(String),
    CreateSubCategory(String),
}

pub enum DialogResult {
    Ok,
    Cancel,
}

pub enum ShowDialogResult {
    None,
    CreatedCategory(String),
    CreatedSubCategory(String),
    CloseDialog,
}

pub fn render_dialog(app: &mut MyApp, ctx: &egui::Context) -> Option<ShowDialogResult> {
    if app.modal_dialog.is_none() {
        return None;
    }

    let mut result = Some(ShowDialogResult::None);

    match app.modal_dialog.as_mut().unwrap() {
        ModalWindowState::Authenticate => {
            if render_authenticate(app, ctx) {
                let password =
                    crate::password_utils::make_password_complient(&app.password.as_bytes());

                let aes_key = AesKey {
                    key: password,
                    iv: app.settings.get_iv(),
                };

                if app.load_from_file(&aes_key) {
                    app.modal_dialog = None;
                    app.authenticated = Some(aes_key);
                    app.auth_window_error_message = None;
                } else {
                    app.auth_window_error_message = "Invalid password".to_string().into()
                }
            }
        }
        ModalWindowState::CreateCategory(category) => {
            if let Some(dialog_result) =
                render_edit_modal(ctx, "Enter category name:", "Add", category)
            {
                if let DialogResult::Ok = dialog_result {
                    app.categories.insert(category.to_string(), BTreeMap::new());
                    app.has_not_saved_data = true;
                    result = Some(ShowDialogResult::CreatedCategory(category.to_string()));
                } else {
                    result = Some(ShowDialogResult::CloseDialog);
                }
            }
        }
        ModalWindowState::UpdateCategory(category) => {
            if let Some(dialog_result) =
                render_edit_modal(ctx, "Enter category name:", "Edit", category)
            {
                if let DialogResult::Ok = dialog_result {}
                result = Some(ShowDialogResult::CloseDialog);
            }
        }

        ModalWindowState::CreateSubCategory(sub_category) => {
            if let Some(dialog_result) =
                render_edit_modal(ctx, "Enter subcategory name:", "Add", sub_category)
            {
                if let DialogResult::Ok = dialog_result {
                    app.categories
                        .get_mut(app.selected_category.as_ref().unwrap())
                        .unwrap()
                        .insert(sub_category.to_string(), "".to_string());

                    app.has_not_saved_data = true;
                }

                result = Some(ShowDialogResult::CreatedSubCategory(
                    sub_category.to_string(),
                ));
            }
        }
    }

    result
}

fn render_authenticate(app: &mut MyApp, ctx: &egui::Context) -> bool {
    let mut result = false;
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("Please enter your password:");
            ui.group(|ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::new(2.0, 10.0);

                ui.add(egui::TextEdit::singleline(&mut app.password).password(true));

                if let Some(auth_err) = &app.auth_window_error_message {
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
