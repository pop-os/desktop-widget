use crate::fl;
use gtk::prelude::*;

pub fn page(header: &gtk::Widget) -> gtk::Widget {
    (cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 0);
        ..set_halign(gtk::Align::Center);
        ..add(header);
        ..add(&gtk::Label::new(Some(&fl!("gis-dock-header"))));
        ..add(&crate::dock_selector());
        ..add(&gtk::Label::new(Some(&fl!("gis-dock-description"))));
    })
    .upcast()
}

pub fn title() -> String { fl!("gis-dock-title") }
