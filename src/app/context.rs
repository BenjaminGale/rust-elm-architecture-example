use crate::app::message::Msg;
use crate::app::model::AppModel;
use crate::view::app::AppView;
use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn dispatcher(self: &Self) -> Dispatcher {
        Dispatcher::new(self.clone())
    }

    fn dispatch<T: Into<Msg>>(self: &Self, msg: T) {
        let msg = msg.into();

        self.model.borrow_mut().update(&msg);
        self.view.borrow_mut().render(&self.model.borrow(), &msg,  &self.dispatcher());
    }
}

#[derive(Clone)]
pub struct Dispatcher<> {
    app_context: AppContext
}

impl Dispatcher {
    fn new(app_context: AppContext) -> Dispatcher {
        Dispatcher {
            app_context
        }
    }

    pub fn dispatch<E: Into<Msg>>(self: &Self, event: E) {
        self.app_context.dispatch(event);
    }
}
