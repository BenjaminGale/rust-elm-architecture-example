use gtk::{Application, ApplicationWindow};
use gtk::prelude::GtkWindowExt;
use crate::app::context::AppContext;
use crate::app::model::AppModel;
use crate::view::counter_view::CounterView;

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

    pub fn show(self: &Self) {
        self.main_window.present();
    }

    fn add_counter_view(self: &mut Self, model: &AppModel, app_context: AppContext) {
        let view = CounterView::new(model, app_context.clone());
        
        self.main_window.set_child(Some(&view.root));
        self.counter_view = Some(view);
    }

    pub fn render(self: &mut Self, model: &AppModel, app_context: AppContext) {
        match &mut self.counter_view {
            None => self.add_counter_view(model, app_context),
            Some(view)  => view.render(model)
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
