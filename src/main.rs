mod components;
mod file;
mod password_utils;
mod render_bottom_panel;
mod settings;
mod states;
mod text_buffer;

use components::*;
use egui::{FontData, FontDefinitions};

use crate::settings::SettingsModel;

use crate::states::*;

const FONT_MESLO: &[u8] = std::include_bytes!("../fonts/MesloLGS NF Regular.ttf");

pub struct MyApp {
    settings: SettingsModel,
    pub authenticated: Option<AuthenticatedState>,
    pub selected_category: Option<String>,
    pub selected_sub_category: Option<SelectedSubCategoryState>,
    pub has_not_saved_data: bool,
    pub modal_dialog: ModalDialog,
    pub edit_state: EditingState,
    //pub normal_style: Rc<Style>,
    //pub category_style: Rc<Style>,
}

impl MyApp {
    fn get_content_ref_mut(&mut self) -> &mut TypeContent {
        &mut self.authenticated.as_mut().unwrap().content
    }
    /*
    fn get_content_by_selected_category_mut(&mut self) -> &mut BTreeMap<String, String> {
        match &self.selected_category {
            Some(selected_category) => {
                let authenticated_state = self.authenticated.as_mut().unwrap();
                authenticated_state
                    .content
                    .get_mut(selected_category)
                    .unwrap()
            }
            None => {
                panic!("There is not selected category")
            }
        }
    }


    fn get_content_by_selected_category(&self) -> &BTreeMap<String, String> {
        match &self.selected_category {
            Some(selected_category) => {
                let authenticated_state = self.authenticated.as_ref().unwrap();
                authenticated_state.content.get(selected_category).unwrap()
            }
            None => {
                panic!("There is not selected category")
            }
        }
    } */

    fn update_edited_content(&mut self) -> &AuthenticatedState {
        let selected_category = match &self.selected_category {
            Some(selected_category) => selected_category,
            None => {
                panic!("There is not selected category")
            }
        };

        let selected_sub_category = match &self.selected_sub_category {
            Some(selected_sub_category) => selected_sub_category.clone(),
            None => {
                panic!("There is not selected category")
            }
        };

        let authenticated_state = self.authenticated.as_mut().unwrap();

        if let Some(sub_level_data) = authenticated_state.content.get_mut(selected_category) {
            sub_level_data.insert(selected_sub_category.id, selected_sub_category.text);
        }

        authenticated_state
    }

    fn get_selected_content(&self, sub_category_id: &str) -> Option<&str> {
        let auth_data = self.authenticated.as_ref()?;

        let selected_category = self.selected_category.as_ref()?;

        let first_level = auth_data.content.get(selected_category)?;

        let result = match first_level.get(sub_category_id) {
            Some(value) => value.as_str(),
            None => "",
        };

        Some(result)
    }

    /*
    pub fn flush_active_subcategory(&mut self) {
        if let Some(active_subcategory) = self.active_sub_category.take() {
            self.get_content_by_selected_category_mut().insert(
                active_subcategory.sub_category_id.clone(),
                active_subcategory.text.clone(),
            );

            self.active_sub_category = Some(active_subcategory);
        }
    }
     */

    pub fn save_to_file(&mut self) {
        let state = self.update_edited_content();
        crate::file::save_to_file(&state.aes_key, &state.content);
        self.edit_state.finish_editing();
        self.has_not_saved_data = false;
    }

    pub fn cancel_not_saved_data(&mut self) {
        let prev_content = self.edit_state.finish_editing();

        if let Some(sub_category_content) = self.selected_sub_category.as_mut() {
            sub_category_content.text = prev_content;
        }

        self.has_not_saved_data = false;
    }

    pub fn select_category(&mut self, category_id: Option<String>) {
        if self.selected_sub_category.is_some() {
            self.select_sub_category(None);
        }

        self.selected_category = category_id;
    }

    pub fn select_sub_category(&mut self, sub_category_id: Option<String>) {
        match sub_category_id {
            Some(sub_category_id) => {
                let content = self.get_selected_content(&sub_category_id).unwrap();
                self.selected_sub_category = Some(SelectedSubCategoryState {
                    id: sub_category_id,
                    text: content.to_string(),
                });
            }
            None => {
                self.selected_sub_category = None;
            }
        }
    }

    pub fn handle_dialog_result(&mut self, dialog_result: ShowDialogResult) {
        match dialog_result {
            ShowDialogResult::DialogIsBeingRendered => {}
            ShowDialogResult::Authenticated { aes_key, data } => {
                self.authenticated = Some(AuthenticatedState {
                    aes_key,
                    content: data,
                });
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
            ShowDialogResult::RenameCategory(new_category_name) => {
                let from = self.selected_category.as_ref().unwrap().to_string();
                self.select_category(None);

                let content = self.get_content_ref_mut();

                let removed = content.remove(&from).unwrap();

                content.insert(new_category_name.clone(), removed);

                self.select_category(Some(new_category_name));
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
            if self.selected_sub_category.is_some() {
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
        selected_category: None,
        modal_dialog: Default::default(),
        selected_sub_category: None,
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
