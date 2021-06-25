use gio::prelude::*;
use gtk::prelude::*;

pub fn page(header: &gtk::Widget) -> gtk::Widget {
    (cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 0);
        ..set_halign(gtk::Align::Center);
        ..add(header);
        ..add(&gtk::Label::new(Some(
            "Continue the desktop setup by choosing your preferred layout."
        )));
        ..add(&crate::dock_selector());
        ..add(&gtk::Label::new(Some(concat!(
            "Dock appearance, its size, and position can be changed at any time ",
            "from the Settings application."
        ))));
    })
    .upcast()
}

pub fn title() -> String {
    // TODO: Localize
    String::from("Welcome to Pop!_OS")
}
