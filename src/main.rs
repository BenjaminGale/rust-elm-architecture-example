use crate::app::context::AppContext;
use crate::app::model::AppModel;
use crate::view::app::AppView;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{glib, Application};

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

    app_context.show_main_window();
}
