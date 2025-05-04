mod components;
mod file;
mod password_utils;
mod render_bottom_panel;
mod settings;
mod states;

use std::collections::BTreeMap;

use components::*;
use egui::{FontData, FontDefinitions, TextBuffer};
use encryption::aes::AesKey;

use crate::settings::SettingsModel;

use crate::states::*;

const FONT_MESLO: &[u8] = std::include_bytes!("../fonts/MesloLGS NF Regular.ttf");

pub struct SelectedSubcategory {
    pub sub_category_id: String,
    pub text: String,
}

pub struct MyApp {
    settings: SettingsModel,
    pub authenticated: Option<AesKey>,

    pub categories: BTreeMap<String, BTreeMap<String, String>>,

    pub selected_category: Option<String>,

    pub active_sub_category: Option<SelectedSubcategory>,

    pub has_not_saved_data: bool,

    pub modal_dialog: ModalDialog,
    pub edit_state: EditingState,
    //pub normal_style: Rc<Style>,
    //pub category_style: Rc<Style>,
}

impl TextBuffer for MyApp {
    fn is_mutable(&self) -> bool {
        self.edit_state.is_editing()
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
        self.edit_state.finish_editing();
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
        self.edit_state.finish_editing();
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

    pub fn handle_dialog_result(&mut self, dialog_result: ShowDialogResult) {
        match dialog_result {
            ShowDialogResult::DialogIsBeingRendered => {}
            ShowDialogResult::Authenticated { aes_key, data } => {
                self.categories = data;
                self.authenticated = Some(aes_key);
                self.edit_state.extend_expiration_time();
                self.modal_dialog.set_none();
            }
            ShowDialogResult::CreatedSubCategory(sub_category) => {
                self.select_sub_category(Some(sub_category));
                self.modal_dialog.set_none();
            }
            ShowDialogResult::CreatedCategory(category) => {
                self.select_category(Some(category));
                self.modal_dialog.set_none();
            }
            ShowDialogResult::RenameCategory(category) => {
                let from = self.selected_category.as_ref().unwrap().to_string();
                self.select_category(None);

                let removed = self.categories.remove(&from).unwrap();

                self.categories.insert(category.clone(), removed);

                self.select_category(Some(category));
                self.has_not_saved_data = true;
                self.modal_dialog.set_none();
            }
            ShowDialogResult::Cancel => {
                self.modal_dialog.set_none();
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(dialog_result) = self.render_dialog(ctx) {
            self.handle_dialog_result(dialog_result);
            return;
        }

        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            if let Some(result) = crate::components::side_panel::render(self, ui) {
                match result {
                    side_panel::SizePanelEvent::SubCategorySelected(sub_category) => {
                        self.select_sub_category(sub_category);
                        self.edit_state.extend_expiration_time();
                    }
                    side_panel::SizePanelEvent::CategorySelected(category) => {
                        self.select_category(Some(category));
                        self.edit_state.extend_expiration_time();
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

        self.render_bottom_panel(ctx);
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

        selected_category: None,
        modal_dialog: Default::default(),
        active_sub_category: None,
        has_not_saved_data: false,
        edit_state: EditingState::new(),
        //normal_style: Rc::new(style.clone()),
        //category_style: Rc::new(category_style),
        settings,
    };

    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;
    //native_options.viewport.inner_size = Some(egui::vec2(1024.0 * 4.0, 768.0 * 4.0));
    //native_options.decorated = true;

    let _ = eframe::run_native(
        "My secrets",
        native_options,
        Box::new(|cc| {
            //cc.egui_ctx.set_zoom_factor(0.5);
            cc.egui_ctx.set_style(style);
            cc.egui_ctx.set_pixels_per_point(1.0);
            configure_fonts(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    );
}

fn configure_fonts(ctx: &egui::Context) {
    let mut font_def = FontDefinitions::default();

    font_def.font_data.insert(
        "MesloLGS".to_string(),
        FontData::from_static(FONT_MESLO).into(),
    );

    font_def
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "MesloLGS".to_owned());

    font_def
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("MesloLGS".to_owned());

    ctx.set_fonts(font_def);
}
