use gio::prelude::*;
use gtk::prelude::*;

pub fn page(header: &gtk::Widget) -> gtk::Widget {
    let description = gtk::LabelBuilder::new()
        .wrap(true)
        .justify(gtk::Justification::Center)
        .label(concat!(
            "Press Super key or use an icon in the dock to display the Launcher search field. Use ",
            "arrow keys to quickly switch between open windows or type the name of the application ",
            "to launch it. The Launcher makes navigating the desktop faster and more fluid."
        ))
        .build();

    let image = gtk::ImageBuilder::new()
        .resource("/org/pop/desktop-widget/launcher.png")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .vexpand(true)
        .margin_top(32)
        .build();

    let extra_notice = gtk::LabelBuilder::new()
        .label("Super key configuration can be changed at any time from the Settings application.")
        .build();

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
    String::from("Open and Switch Applications from Launcher")
}
