use crate::app::event::Event;
use crate::app::model::{update_model, AppModel};
use std::cell::RefCell;
use std::rc::Rc;
use crate::view::app_view::AppView;

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

    pub fn dispatch<T: Into<Event>>(self: &Self, event: T) {
        update_model(&mut self.model.borrow_mut(), &event.into());
        self.view.borrow_mut().render(&self.model.borrow(), self.clone());
    }
}
