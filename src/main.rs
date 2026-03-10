use std::cell::{RefCell};
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib, Button, Label, Align};

#[derive(Debug)]
struct AppState {
    count: isize
}

impl AppState {

    fn new() -> AppState {
        AppState {
            count: 0
        }
    }
}

enum Event {
    Init,
    Increment,
    Decrement,
}

fn dispatch(app_state: Rc<RefCell<AppState>>, event: Event, label: Rc<RefCell<Label>>) {
    update(&mut app_state.borrow_mut(), &event);
    render(&app_state.borrow(), label.clone());
}

fn update(app_state: &mut AppState, event: &Event) {
    match event {
        Event::Init => return,
        Event::Increment => app_state.count += 1,
        Event::Decrement => app_state.count -= 1,
    }
}

fn render(app_state: &AppState, label: Rc<RefCell<Label>>) {
    label.borrow_mut().set_label(format!("Count: {:?}", app_state.count).as_str());
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("Gtk.Elm.Architecture")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let app_state = Rc::new(RefCell::new(AppState::new()));

    let label = Label::builder()
        .label("")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_inc = Button::builder()
        .label("+")
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_dec = Button::builder()
        .label("-")
        .margin_start(12)
        .margin_end(12)
        .build();

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .build();

    container.append(&label);
    container.append(&button_inc);
    container.append(&button_dec);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Elm Architecture Example")
        .default_width(350)
        .default_height(150)
        .resizable(false)
        .child(&container)
        .build();

    let label_ref = Rc::new(RefCell::new(label));

    button_inc.connect_clicked({
        let inc_state = app_state.clone();
        let lbl = label_ref.clone();
        move |_| {
            dispatch(inc_state.clone(), Event::Increment, lbl.clone());
        }
    });

    button_dec.connect_clicked({
        let dec_state = app_state.clone();
        let lbl = label_ref.clone();
        move |_| {
            dispatch(dec_state.clone(), Event::Decrement, lbl.clone());
        }
    });

    dispatch(app_state.clone(), Event::Init, label_ref.clone());

    window.present();
}
