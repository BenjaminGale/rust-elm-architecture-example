use gtk::{Application, ApplicationWindow};
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

    pub fn render(self: &mut Self, model: &AppModel, app_context: AppContext) {
        match &mut self.counter_view {
            None => self.counter_view = Some(CounterView::new(model, &self.main_window, app_context.clone())),
            Some(counter)  => counter.render(model)
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
