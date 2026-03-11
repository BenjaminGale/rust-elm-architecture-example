use crate::app::event::Event;

#[derive(Debug)]
pub struct AppModel {
    pub count: isize
}

impl AppModel {
    pub fn new() -> AppModel {
        AppModel {
            count: 0
        }
    }

    pub fn format_count(self: &Self) -> String {
        format!("Count: {}", self.count)
    }
}

pub fn update_model(model: &mut AppModel, event: &Event) {
    match event {
        Event::Init => return,
        Event::Increment => model.count += 1,
        Event::Decrement => model.count -= 1,
    }
}
