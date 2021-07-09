use crate::fl;
use gio::prelude::*;
use gtk::prelude::*;

pub fn page(header: &gtk::Widget) -> gtk::Widget {
    let description = gtk::LabelBuilder::new()
        .wrap(true)
        .justify(gtk::Justification::Center)
        .label(&fl!("gis-gestures-description"))
        .build();

    let video = gtk::ImageBuilder::new()
        .resource("/org/pop/desktop-widget/gestures.png")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .vexpand(true)
        .margin_top(32)
        .build()
        .upcast::<gtk::Widget>();

    (cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 0);
        ..set_halign(gtk::Align::Center);
        ..add(header);
        ..add(&description);
        ..add(&video);
    })
    .upcast()
}

pub fn title() -> String { fl!("gis-gestures-title") }
