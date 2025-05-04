use crate::MyApp;

pub enum SizePanelEvent {
    CategorySelected(String),
    SubCategorySelected(Option<String>),
}

pub fn render(app: &mut MyApp, ui: &mut egui::Ui) -> Option<SizePanelEvent> {
    let mut result = None;

    let is_editing = app.edit_state.is_editing();

    let authenticated = app.authenticated.as_ref()?;

    for (category, sub_categories) in &authenticated.content {
        //   ui.set_style(app.category_style.clone());

        ui.vertical_centered_justified(|ui| {
            let text = egui::RichText::new(category);
            let widget_text = egui::WidgetText::RichText(text).monospace().heading();

            let checked = if let Some(selected_category) = &app.selected_category {
                selected_category == category
            } else {
                false
            };

            let response = ui.selectable_label(checked, widget_text);

            if !is_editing {
                if response.clicked() {
                    result = Some(SizePanelEvent::CategorySelected(category.to_string()));
                }
            }
        });

        if let Some(selected_category) = &app.selected_category {
            if category == selected_category {
                for (sub_category, _) in sub_categories {
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.text_style_height(&egui::TextStyle::Monospace);
                            ui.label("⇢ ");

                            let checked =
                                if let Some(selected_sub_category) = &app.selected_sub_category {
                                    &selected_sub_category.id == sub_category
                                } else {
                                    false
                                };

                            let response = ui.selectable_label(checked, sub_category);

                            if !is_editing {
                                if response.clicked() && !is_editing {
                                    result = Some(SizePanelEvent::SubCategorySelected(Some(
                                        sub_category.to_string(),
                                    )));
                                }
                            }
                        })
                    });
                }

                ui.separator();
            }
        }
    }

    result
}
