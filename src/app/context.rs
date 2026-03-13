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

    fn dispatch<T: Into<Msg>>(self: &Self, event: T) {
        let dispatcher = Dispatcher::new(self.clone());

        self.model.borrow_mut().update(&event.into());
        self.view.borrow_mut().render(&self.model.borrow(), &dispatcher);
    }
}

#[derive(Clone)]
pub struct Dispatcher<> {
    app_context: AppContext
}

impl Dispatcher {
    pub fn new(app_context: AppContext) -> Dispatcher {
        Dispatcher {
            app_context
        }
    }

    pub fn dispatch<E: Into<Msg>>(self: &Self, event: E) {
        self.app_context.dispatch(event);
    }
}
