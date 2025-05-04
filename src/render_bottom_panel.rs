use crate::{states::*, MyApp};
use native_dialog::MessageDialog;

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

                        if self.active_sub_category.is_some() {
                            if ui.small_button("Edit").clicked() {
                                self.edit_state.start_editing();
                            };
                        }
                    }
                }

                if self.has_not_saved_data {
                    if ui.small_button("Save").clicked() {
                        let confirm = MessageDialog::new()
                            .set_type(native_dialog::MessageType::Warning)
                            .set_title("Confirmation")
                            .set_text("Please confirm that you want to save the changes.")
                            .show_confirm()
                            .unwrap();

                        if confirm {
                            self.save_to_file();
                        }
                    };

                    if ui.small_button("Cancel").clicked() {
                        let confirm = MessageDialog::new()
                            .set_type(native_dialog::MessageType::Warning)
                            .set_title("Confirmation")
                            .set_text("Please confirm that you want to cancel the changes.")
                            .show_confirm()
                            .unwrap();

                        if confirm {
                            self.cancel_not_saved_data();
                        }
                    };
                }
            });
        });
    }
}
