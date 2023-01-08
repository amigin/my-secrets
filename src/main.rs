mod components;
mod file;
mod password_utils;
mod settings;

use std::{collections::BTreeMap, sync::Arc};

use components::*;
use egui::{FontData, FontDefinitions, Style, TextBuffer};
use encryption::aes::AesKey;
use native_dialog::MessageDialog;

use crate::settings::SettingsModel;

const FONT_MESLO: &[u8] = std::include_bytes!("../fonts/MesloLGS NF Regular.ttf");

pub struct SelectedSubcategory {
    pub sub_category_id: String,
    pub text: String,
}

pub struct MyApp {
    settings: SettingsModel,
    pub authenticated: Option<AesKey>,

    pub auth_window_error_message: Option<String>,

    pub password: String,
    pub categories: BTreeMap<String, BTreeMap<String, String>>,

    pub selected_category: Option<String>,

    pub active_sub_category: Option<SelectedSubcategory>,

    pub has_not_saved_data: bool,

    pub modal_dialog: Option<ModalWindowState>,
    pub editing: bool,

    pub normal_style: Arc<Style>,
    pub category_style: Arc<Style>,
}

impl TextBuffer for MyApp {
    fn is_mutable(&self) -> bool {
        self.editing
    }

    fn as_str(&self) -> &str {
        self.active_sub_category.as_ref().unwrap().text.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.has_not_saved_data = true;
        self.active_sub_category
            .as_mut()
            .unwrap()
            .text
            .insert_text(text, char_index)
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.has_not_saved_data = true;
        self.active_sub_category
            .as_mut()
            .unwrap()
            .text
            .delete_char_range(char_range)
    }
}

impl MyApp {
    pub fn flush_active_subcategory(&mut self) {
        if let Some(active_subcategory) = self.active_sub_category.take() {
            self.categories
                .get_mut(self.selected_category.as_ref().unwrap())
                .unwrap()
                .insert(
                    active_subcategory.sub_category_id.clone(),
                    active_subcategory.text.clone(),
                );

            self.active_sub_category = Some(active_subcategory);
        }
    }

    pub fn save_to_file(&mut self) {
        self.flush_active_subcategory();
        if let Some(aes_key) = &self.authenticated {
            crate::file::save_to_file(aes_key, &self.categories);
        }

        self.has_not_saved_data = false;
        self.editing = false;
    }

    pub fn cancel_not_saved_data(&mut self) {
        if let Some(selected_sub_category) = &self.active_sub_category {
            self.categories
                .get_mut(self.selected_category.as_ref().unwrap())
                .unwrap()
                .insert(
                    selected_sub_category.sub_category_id.to_string(),
                    selected_sub_category.text.clone(),
                );
        }

        self.has_not_saved_data = false;
        self.editing = false;
    }

    pub fn load_from_file(&mut self, aes_key: &AesKey) -> bool {
        if let Some(result) = crate::file::load_file(aes_key) {
            self.categories = result;
            true
        } else {
            false
        }
    }

    pub fn select_category(&mut self, category_id: Option<String>) {
        if self.active_sub_category.is_some() {
            self.select_sub_category(None);
        }

        self.selected_category = category_id;
    }

    pub fn select_sub_category(&mut self, sub_category_id: Option<String>) {
        let prev_subcategory = self.active_sub_category.take();
        if let Some(prev_subcategory) = prev_subcategory {
            self.categories
                .get_mut(self.selected_category.as_ref().unwrap())
                .unwrap()
                .insert(prev_subcategory.sub_category_id, prev_subcategory.text);
        }

        if let Some(sub_category_id) = sub_category_id {
            let selected_category = self
                .categories
                .get(self.selected_category.as_ref().unwrap());

            let text = selected_category
                .unwrap()
                .get(&sub_category_id)
                .unwrap()
                .to_string();

            self.active_sub_category = Some(crate::SelectedSubcategory {
                sub_category_id,
                text,
            });
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let show_dialog_result = crate::render_dialog(self, ctx);

        if let Some(show_dialog_result) = show_dialog_result {
            match show_dialog_result {
                ShowDialogResult::None => {}
                ShowDialogResult::CreatedSubCategory(sub_category) => {
                    self.select_sub_category(Some(sub_category));
                    self.modal_dialog = None;
                }
                ShowDialogResult::CloseDialog => {
                    self.modal_dialog = None;
                }
                ShowDialogResult::CreatedCategory(category) => {
                    self.select_category(Some(category));
                    self.modal_dialog = None;
                }
            }
            return;
        }

        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            if let Some(result) = crate::components::side_panel::render(self, ui) {
                match result {
                    side_panel::SizePanelEvent::SubCategorySelected(sub_category) => {
                        self.select_sub_category(sub_category);
                    }
                    side_panel::SizePanelEvent::CategorySelected(category) => {
                        self.select_category(Some(category));
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.active_sub_category.is_some() {
                // let width = ui.available_width();
                let h = ui.available_height();
                //println!(" {}x{}", width, h);
                //ui.allocate_space(ui.available_size());
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        ui.set_max_height(h - 18.0);

                        egui::ScrollArea::vertical()
                            .auto_shrink([true, true])
                            //.always_show_scroll(true)
                            .show(ui, |ui| {
                                // ui.set_width(width - 2.0);
                                let text_edit = egui::TextEdit::multiline(self);

                                ui.add(text_edit);
                            });
                    },
                );
            }
        });
        egui::TopBottomPanel::bottom("bottom panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if !self.editing {
                    if ui.small_button("Add category").clicked() {
                        self.modal_dialog = Some(ModalWindowState::CreateCategory("".to_string()));
                    };

                    if let Some(selected_category) = &self.selected_category {
                        if ui.small_button("Edit category").clicked() {
                            self.modal_dialog =
                                Some(ModalWindowState::UpdateCategory(selected_category.clone()));
                        };

                        if ui.small_button("Add subcategory").clicked() {
                            self.modal_dialog =
                                Some(ModalWindowState::CreateSubCategory("".to_string()));
                        };

                        if ui.small_button("Edit").clicked() {
                            self.editing = true;
                        };
                    }
                } else {
                    if !self.has_not_saved_data {
                        if ui.small_button("Stop editing").clicked() {
                            self.editing = false;
                        };
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

fn main() {
    let settings = SettingsModel::read();

    if settings.shared_key.as_bytes().len() != 16 {
        panic!("Shared key must be 16 bytes long");
    }

    let style = egui::Style {
        visuals: egui::Visuals::light(),
        ..egui::Style::default()
    };

    let mut category_style = style.clone();

    category_style.override_text_style = Some(egui::TextStyle::Heading);
    category_style.visuals.selection.bg_fill = egui::Color32::from_rgb(0, 0, 255);
    category_style.visuals.selection.stroke.color = egui::Color32::from_rgb(255, 255, 255);

    let app = MyApp {
        authenticated: None,
        categories: BTreeMap::new(),
        password: String::new(),
        selected_category: None,
        modal_dialog: Some(ModalWindowState::Authenticate),
        active_sub_category: None,
        has_not_saved_data: false,
        editing: false,
        normal_style: Arc::new(style.clone()),
        category_style: Arc::new(category_style),
        settings,
        auth_window_error_message: None,
    };

    let mut native_options = eframe::NativeOptions::default();
    native_options.follow_system_theme = true;
    native_options.centered = true;
    native_options.decorated = true;

    eframe::run_native(
        "My secrets",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_style(style);
            cc.egui_ctx.set_pixels_per_point(2.0);
            cc.egui_ctx.set_fonts(configure_fonts());
            Box::new(app)
        }),
    );
}

fn configure_fonts() -> FontDefinitions {
    let mut font_def = FontDefinitions::default();

    font_def
        .font_data
        .insert("MesloLGS".to_string(), FontData::from_static(FONT_MESLO));

    font_def
}
