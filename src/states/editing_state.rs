use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct EditingState {
    editing: bool,
    last_active: DateTimeAsMicroseconds,
}

impl EditingState {
    pub fn new() -> Self {
        Self {
            editing: false,
            last_active: DateTimeAsMicroseconds::now(),
        }
    }

    pub fn is_editing(&self) -> bool {
        self.editing
    }

    pub fn finish_editing(&mut self) {
        self.editing = false;
    }

    pub fn extend_expiration_time(&mut self) {
        self.last_active = DateTimeAsMicroseconds::now();
    }

    pub fn start_editing(&mut self) {
        self.editing = true;
        self.last_active = DateTimeAsMicroseconds::now();
    }

    pub fn activity_expired(&mut self) -> bool {
        if self.editing {
            return false;
        }

        let now = DateTimeAsMicroseconds::now();
        now.duration_since(self.last_active).get_full_minutes() >= 10
    }
}
