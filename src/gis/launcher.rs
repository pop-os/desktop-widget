use crate::fl;
use gio::prelude::*;
use gtk::prelude::*;

pub fn page(header: &gtk::Widget) -> gtk::Widget {
    let description = gtk::LabelBuilder::new()
        .wrap(true)
        .justify(gtk::Justification::Center)
        .label(&fl!("gis-launcher-description"))
        .build();

    let image = gtk::ImageBuilder::new()
        .resource("/org/pop/desktop-widget/launcher.png")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .vexpand(true)
        .margin_top(32)
        .build();

    let extra_notice = gtk::LabelBuilder::new().label(&fl!("gis-launcher-notice")).build();

    (cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 0);
        ..set_halign(gtk::Align::Center);
        ..add(header);
        ..add(&description);
        ..add(&image);
        ..add(&extra_notice);
    })
    .upcast()
}

pub fn title() -> String {
    fl!("gis-launcher-title")
}
