mod app;
mod gui;

use crate::gui::{build_main_window, GuiState};
use app::{dispatch, AppState, Event};
use gtk::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, ButtonExt, GtkWindowExt};
use gtk::{glib, Application};
use std::cell::RefCell;
use std::rc::Rc;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("Gtk.Elm.Architecture")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let main_window = build_main_window(&app);

    let app_state = Rc::new(RefCell::new(AppState::new()));
    let gui_state = Rc::new(RefCell::new(GuiState::new(main_window)));

    dispatch(gui_state.clone(), app_state.clone(), Event::Init);
}
