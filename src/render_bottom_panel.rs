use rfd::{MessageDialog, MessageDialogResult, MessageLevel};

use crate::{states::*, MyApp};

impl MyApp {
    pub fn render_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if self.edit_state.is_editing() {
                    if !self.has_not_saved_data {
                        if ui.small_button("Stop editing").clicked() {
                            self.edit_state.finish_editing();
                        };
                    }
                } else {
                    if ui.small_button("Add category").clicked() {
                        self.modal_dialog
                            .set(ModalWindowState::CreateCategory("".to_string()));
                    };

                    if let Some(selected_category) = &self.selected_category {
                        if ui.small_button("Rename category").clicked() {
                            self.modal_dialog
                                .set(ModalWindowState::RenameCategory(selected_category.clone()));
                        };

                        if ui.small_button("Add subcategory").clicked() {
                            self.modal_dialog
                                .set(ModalWindowState::CreateSubCategory("".to_string()));
                        };

                        if let Some(selected_sub_category) = self.selected_sub_category.as_ref() {
                            if ui.small_button("Edit").clicked() {
                                self.edit_state
                                    .start_editing(selected_sub_category.text.to_string());
                            };
                        }
                    }
                }

                if self.has_not_saved_data {
                    if ui.small_button("Save").clicked() {
                        let dialog_result = MessageDialog::new()
                            .set_level(rfd::MessageLevel::Warning)
                            .set_title("Confirmation")
                            .set_buttons(rfd::MessageButtons::YesNo)
                            .set_description("Please confirm that you want to save the changes.")
                            .show();

                        if let MessageDialogResult::Yes = dialog_result {
                            self.save_to_file();
                        }
                    };

                    if ui.small_button("Cancel").clicked() {
                        let dialog_result = MessageDialog::new()
                            .set_level(MessageLevel::Warning)
                            .set_title("Confirmation")
                            .set_buttons(rfd::MessageButtons::YesNo)
                            .set_description("Please confirm that you want to cancel the changes.")
                            .show();

                        if let MessageDialogResult::Yes = dialog_result {
                            self.cancel_not_saved_data();
                        }
                    };
                }
            });
        });
    }
}
