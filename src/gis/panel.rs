use crate::fl;
use gio::prelude::*;
use gtk::prelude::*;

pub fn page(header: &gtk::Widget) -> gtk::Widget {
    let extra_notice = gtk::LabelBuilder::new().label(&fl!("gis-panel-notice")).build();

    let framed_box = cascade! {
        crate::framed_list_box();
        ..set_margin_top(32);
        ..set_vexpand(true);
        ..set_valign(gtk::Align::Start);
    };

    crate::top_bar(&framed_box);

    (cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 0);
        ..set_halign(gtk::Align::Center);
        ..add(header);
        ..add(&framed_box);
        ..add(&extra_notice);
    })
    .upcast()
}

pub fn title() -> String { fl!("gis-panel-title") }
