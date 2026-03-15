use crate::app::context::Dispatcher;
use crate::app::model::AppModel;
use crate::view::counter::CounterView;
use crate::view::view::LazyView;
use gtk::prelude::{GtkWindowExt, WidgetExt};
use gtk::{Application, ApplicationWindow};

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

    pub fn render(self: &mut Self, model: &AppModel, dispatcher: &Dispatcher) {
        self.counter_view.render(model, dispatcher);

        if let Some(view) = &self.counter_view {
            if view.root.parent().is_none() {
                self.main_window.set_child(Some(&view.root));
                self.main_window.present();
            }
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
