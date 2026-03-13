use gtk::Button;
use gtk::prelude::ButtonExt;
use crate::app::context::AppContext;
use crate::app::message::Msg;

pub trait ButtonDispatcher {
    fn on_clicked<E, F>(&self, app_context: &AppContext, event: F)
    where
        F: Fn() -> E + 'static,
        E: Into<Msg>;
}

impl ButtonDispatcher for Button {
    fn on_clicked<E, F>(&self, app_context: &AppContext, event: F)
        where
            F: Fn() -> E + 'static,
            E: Into<Msg>
    {
        let ctx = app_context.clone();
        self.connect_clicked(move |_| {
            ctx.dispatch(event().into());
        });
    }
}
