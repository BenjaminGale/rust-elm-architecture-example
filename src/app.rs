use crate::{GuiState};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use std::cell::RefCell;
use std::rc::Rc;
use crate::gui::{build_button, build_label, build_layout};

#[derive(Debug)]
pub struct AppState {
    count: isize
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            count: 0
        }
    }

    pub fn create_count_string(self: &Self) -> String {
        format!("Count: {}", self.count)
    }
}

#[derive(Copy, Clone)]
pub enum Event {
    Init,
    Increment,
    Decrement,
}

pub fn dispatch(gui_state: Rc<RefCell<GuiState>>, app_state: Rc<RefCell<AppState>>, event: Event) {
    update_app_state(&mut app_state.borrow_mut(), &event);
    update_gui_state(gui_state, app_state);
}

fn update_app_state(app_state: &mut AppState, event: &Event) {
    match event {
        Event::Init => return,
        Event::Increment => app_state.count += 1,
        Event::Decrement => app_state.count -= 1,
    }
}

fn update_gui_state(gui_state: Rc<RefCell<GuiState>>, app_state: Rc<RefCell<AppState>>) {
    let mut gui = gui_state.borrow_mut();
    let state = app_state.borrow();

    match &mut *gui {
        GuiState::Uninitialised { main_window } => {
            let label = build_label(&state.create_count_string());
            let button_inc = build_button("+");
            let button_dec = build_button("-");

            let container = build_layout();
            container.append(&label);
            container.append(&button_inc);
            container.append(&button_dec);

            button_inc.connect_clicked({
                let state = app_state.clone();
                let gui = gui_state.clone();
                move |_| {
                    dispatch(gui.clone(), state.clone(), Event::Increment);
                }
            });

            button_dec.connect_clicked({
                let state = app_state.clone();
                let gui = gui_state.clone();
                move |_| {
                    dispatch(gui.clone(), state.clone(), Event::Decrement);
                }
            });

            main_window.set_child(Some(&container));
            main_window.present();

            *gui = GuiState::Initialised {
                count_label: label,
            }
        }
        GuiState::Initialised { count_label, .. } => {
            count_label.set_label(&state.create_count_string());
        }
    }
}
