use super::*;

#[derive(Debug)]
pub enum ModalWindowState {
    Authenticate(AuthenticateState),
    CreateCategory(String),
    RenameCategory(String),
    CreateSubCategory(String),
    None,
}

pub struct ModalDialog {
    inner: ModalWindowState,
}

impl ModalDialog {
    pub fn get_mut(&mut self) -> &mut ModalWindowState {
        &mut self.inner
    }

    pub fn set(&mut self, value: ModalWindowState) {
        self.inner = value;
    }

    pub fn set_none(&mut self) {
        self.inner = ModalWindowState::None;
    }
}

impl Default for ModalDialog {
    fn default() -> Self {
        Self {
            inner: ModalWindowState::Authenticate(Default::default()),
        }
    }
}
