use crate::app::context::AppContext;
use crate::app::message::CounterMsg;
use crate::app::model::AppModel;
use gtk::prelude::BoxExt;
use gtk::{glib, Align, Button, Label};
use crate::view::button_ext::ButtonDispatcher;

pub struct CounterView {
    pub root: gtk::Box,
    label: Label,
}

impl CounterView {
    pub fn new(model: &AppModel, app_context: &AppContext) -> CounterView {
        let label = build_label(&model.format_count());

        let inc_button = build_button("+");
        let dec_button = build_button("-");

        inc_button.on_clicked(app_context, || CounterMsg::Increment);
        dec_button.on_clicked(app_context, || CounterMsg::Decrement);

        let container = build_layout();
        container.append(&label);
        container.append(&inc_button);
        container.append(&dec_button);

        CounterView {
            root: container,
            label
        }
    }

    pub fn render(self: &mut Self, model: &AppModel) {
        self.label.set_label(&model.format_count());
    }
}

fn build_label(text: &str) -> Label {
    Label::builder()
        .label(text)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

fn build_button<T: Into<glib::GString>>(label: T) -> Button {
    Button::builder()
        .label(label)
        .margin_start(12)
        .margin_end(12)
        .build()
}

fn build_layout() -> gtk::Box {
    gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .build()
}
