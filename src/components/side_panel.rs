use crate::MyApp;

pub enum SizePanelEvent {
    CategorySelected(String),
    SubCategorySelected(Option<String>),
}

pub fn render(app: &mut MyApp, ui: &mut egui::Ui) -> Option<SizePanelEvent> {
    let mut result = None;

    for (category, sub_categories) in &app.categories {
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

            if response.clicked() && !app.editing {
                result = Some(SizePanelEvent::CategorySelected(category.to_string()));
            }
        });

        if let Some(selected_category) = &app.selected_category {
            if category == selected_category {
                for (sub_category, _) in sub_categories {
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.text_style_height(&egui::TextStyle::Monospace);
                            ui.label("⇢ ");

                            let checked = if let Some(selected) = &app.active_sub_category {
                                &selected.sub_category_id == sub_category
                            } else {
                                false
                            };

                            let response = ui.selectable_label(checked, sub_category);

                            if response.clicked() && !app.editing {
                                if let Some(selected_sub_category) = &app.active_sub_category {
                                    if &selected_sub_category.sub_category_id == sub_category {
                                        result = Some(SizePanelEvent::SubCategorySelected(None));
                                    } else {
                                        result = Some(SizePanelEvent::SubCategorySelected(Some(
                                            sub_category.to_string(),
                                        )));
                                    }
                                } else {
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
