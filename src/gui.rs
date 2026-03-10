use gtk::{glib, Align, Application, ApplicationWindow, Button, Label};

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
