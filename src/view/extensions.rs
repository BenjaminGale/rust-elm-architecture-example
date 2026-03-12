use gtk::Button;
use gtk::prelude::ButtonExt;
use crate::app::context::AppContext;
use crate::app::event::Event;

pub trait ButtonExtensions {
    fn on_button_clicked<T>(&self, app_context: AppContext, event: T)
        where T: Into<Event> + Clone + 'static;
}

impl ButtonExtensions for Button {
    fn on_button_clicked<T>(&self, app_context: AppContext, event: T)
        where T: Into<Event> + Clone + 'static
    {
        self.connect_clicked(move |_| {
            app_context.dispatch(event.clone().into());
        });
    }
}
