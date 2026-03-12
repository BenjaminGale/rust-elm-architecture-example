use crate::app::context::AppContext;
use crate::app::event::AppEvent;
use crate::app::model::AppModel;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{glib, Application};
use crate::view::app_view::AppView;

mod app;
mod view;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("Gtk.Elm.Architecture")
        .build();

    app.connect_activate(on_activate);
    app.run()
}

fn on_activate(app: &Application) {
    let model = AppModel::new();
    let view = AppView::new(app);
    let app_context = AppContext::new(model, view);

    app_context.dispatch(AppEvent::Init);
}
