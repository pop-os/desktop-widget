use gio::prelude::*;
use gtk::prelude::*;

pub fn page(header: &gtk::Widget) -> gtk::Widget {
    let description = gtk::LabelBuilder::new()
        .wrap(true)
        .justify(gtk::Justification::Center)
        .label(concat!(
            "Use four finger swipe left to open Workspaces and windows overview, four fingers swipe ",
            "right to open Applications, and four fingers swipe up or down to switch between ",
            "workspaces. Swipe with three fingers to switch between windows."
        ))
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

pub fn title() -> String {
    String::from("Use Gestures for Easier Navigation")
}
