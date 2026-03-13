use crate::app::message::{AppMsg, Msg};
use crate::app::model::{AppModel};
use std::cell::RefCell;
use std::rc::Rc;
use crate::view::app::AppView;

#[derive(Clone)]
pub struct AppContext {
    view: Rc<RefCell<AppView>>,
    model: Rc<RefCell<AppModel>>
}

impl AppContext {
    pub fn new(model: AppModel, view: AppView) -> AppContext {
        AppContext {
            view: Rc::new(RefCell::new(view)),
            model: Rc::new(RefCell::new(model))
        }
    }

    pub fn show_main_window(self: &Self) {
        self.dispatch(AppMsg::Init);
        self.view.borrow().show();
    }

    pub fn dispatch<T: Into<Msg>>(self: &Self, event: T) {
        self.model.borrow_mut().update(&event.into());
        self.view.borrow_mut().render(&self.model.borrow(), self);
    }
}
