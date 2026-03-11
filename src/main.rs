mod app;
mod gui;
mod model;

use crate::app::AppContext;
use crate::gui::{build_main_window, GuiState};
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::{glib, Application};
use crate::model::Event;

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
