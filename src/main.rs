use gtk::{glib, Application};
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use crate::app::context::AppContext;
use crate::app::event::Event;
use crate::gui::build_main_window;

mod app;
mod gui;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("Gtk.Elm.Architecture")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let main_window = build_main_window(&app);
    let app_context = AppContext::new(main_window);

    app_context.dispatch(Event::Init);
}
