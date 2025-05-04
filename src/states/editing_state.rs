use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct EditingState {
    editing: Option<String>,
    last_active: DateTimeAsMicroseconds,
}

impl EditingState {
    pub fn new() -> Self {
        Self {
            editing: None,
            last_active: DateTimeAsMicroseconds::now(),
        }
    }

    pub fn is_editing(&self) -> bool {
        self.editing.is_some()
    }

    pub fn finish_editing(&mut self) -> String {
        self.editing.take().unwrap()
    }

    pub fn extend_expiration_time(&mut self) {
        self.last_active = DateTimeAsMicroseconds::now();
    }

    pub fn start_editing(&mut self, current_text: String) {
        self.editing = Some(current_text);
        self.last_active = DateTimeAsMicroseconds::now();
    }

    pub fn activity_expired(&mut self) -> bool {
        if self.editing.is_some() {
            return false;
        }

        let now = DateTimeAsMicroseconds::now();
        now.duration_since(self.last_active).get_full_minutes() >= 10
    }

    pub fn editing_value(&self) -> &str {
        match self.editing.as_ref() {
            Some(value) => value,
            None => "",
        }
    }

    pub fn editing_value_mut(&self) -> &str {
        self.editing.as_ref().unwrap()
    }
}
