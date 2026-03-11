use std::cell::RefCell;
use std::rc::Rc;
use gtk::ApplicationWindow;
use crate::app::event::Event;
use crate::app::model::{update_model, AppModel};
use crate::gui::{render, GuiState};

pub struct AppContext {
    gui: Rc<RefCell<GuiState>>,
    model: Rc<RefCell<AppModel>>
}

impl AppContext {
    pub fn new(main_window: ApplicationWindow) -> AppContext {
        AppContext {
            gui: Rc::new(RefCell::new(GuiState::new(main_window))),
            model: Rc::new(RefCell::new(AppModel::new()))
        }
    }

    pub fn dispatch(self: &Self, event: Event) {
        update_model(&mut self.model.borrow_mut(), &event);
        render(&mut self.gui.borrow_mut(), &self.model.borrow(), self.clone());
    }
}

impl Clone for AppContext {
    fn clone(&self) -> Self {
        Self {
            gui: self.gui.clone(),
            model: self.model.clone(),
        }
    }
}
