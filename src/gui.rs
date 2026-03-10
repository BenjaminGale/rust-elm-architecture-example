use std::cell::RefCell;
use std::rc::Rc;
use gtk::{glib, Align, Application, ApplicationWindow, Button, Label};
use gtk::prelude::ButtonExt;
use crate::app::{dispatch, AppState, Event};

pub enum GuiState {
    Uninitialised {
        main_window: ApplicationWindow,
    },
    Initialised {
        count_label: Label
    }
}

impl GuiState {
    pub fn new(main_window: ApplicationWindow) -> GuiState {
        GuiState::Uninitialised {
            main_window
        }
    }
}

pub fn build_button<T: Into<glib::GString>>(label: T) -> Button {
    Button::builder()
        .label(label)
        .margin_start(12)
        .margin_end(12)
        .build()
}

pub fn build_layout() -> gtk::Box {
    gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .build()
}

pub fn build_label(text: &str) -> Label {
    Label::builder()
        .label(text)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

pub fn build_main_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("Elm Architecture Example")
        .default_width(350)
        .default_height(150)
        .resizable(false)
        .build()
}

pub trait ButtonExtensions {
    fn on_button_clicked(&self, app_state: Rc<RefCell<AppState>>, gui_state: Rc<RefCell<GuiState>>, event: Event);
}

impl ButtonExtensions for Button {
    fn on_button_clicked(&self, app_state: Rc<RefCell<AppState>>, gui_state: Rc<RefCell<GuiState>>, event: Event) {
        self.connect_clicked(move |_| {
            dispatch(gui_state.clone(), app_state.clone(), event);
        });
    }
}
