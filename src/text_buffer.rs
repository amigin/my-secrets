use egui::TextBuffer;

use crate::MyApp;

impl TextBuffer for MyApp {
    fn is_mutable(&self) -> bool {
        self.edit_state.is_editing()
    }

    fn as_str(&self) -> &str {
        self.selected_sub_category.as_ref().unwrap().text.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.has_not_saved_data = true;
        self.selected_sub_category
            .as_mut()
            .unwrap()
            .text
            .insert_text(text, char_index)
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.has_not_saved_data = true;
        self.selected_sub_category
            .as_mut()
            .unwrap()
            .text
            .delete_char_range(char_range)
    }
}
