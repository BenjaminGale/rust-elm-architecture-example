use crate::app::context::Dispatcher;
use crate::app::model::AppModel;
use crate::view::counter::CounterView;
use gtk::prelude::GtkWindowExt;
use gtk::{Application, ApplicationWindow};
use crate::app::message::{AppMsg, Msg};

pub struct AppView {
    main_window: ApplicationWindow,
    counter_view: Option<CounterView>
}

impl AppView {
    pub fn new(app: &Application) -> AppView {
        AppView {
            main_window: build_main_window(app),
            counter_view: None
        }
    }

    pub fn render(self: &mut Self, model: &AppModel, msg: &Msg, dispatcher: &Dispatcher) {
        match &mut self.counter_view {
            None => {
                if let Msg::App(AppMsg::Init) = msg {
                    let view = CounterView::new(model, dispatcher);
                    let root = &view.root;

                    self.main_window.set_child(Some(root));
                    self.counter_view = Some(view);

                    self.main_window.present();
                }
            },
            Some(view) => view.render(model),
        }
    }
}

fn build_main_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title("Counter App")
        .default_width(350)
        .default_height(150)
        .resizable(false)
        .build()
}
