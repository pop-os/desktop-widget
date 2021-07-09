use crate::fl;
use gtk::prelude::*;
use std::{fs::File, path::PathBuf};

pub fn page(header: &gtk::Widget) -> Option<gtk::Widget> {
    let gnome_shell_data_dir = gnome_shell_data_dir();
    let extensions_source = gnome_shell_data_dir.join("extensions");
    let extensions_backup = gnome_shell_data_dir.join("extensions.bak");
    let extensions_notified = extensions_backup.join("notified");

    if !extensions_backup.exists() || extensions_notified.exists() {
        return None;
    }

    let _ = File::create(extensions_notified);

    let label_create = |selectable: bool, label: &str| -> gtk::Label {
        gtk::LabelBuilder::new()
            .justify(gtk::Justification::Center)
            .label(label)
            .selectable(selectable)
            .wrap(true)
            .build()
    };

    let url = "<a href=\"https://extensions.gnome.org/\">extensions.gnome.org</a>";

    let label1 = cascade! {
        label_create(false, &fl!("gis-extensions-label1", url=url));
        ..set_use_markup(true);
        ..connect_activate_link(|_, uri| {
            let _ = std::process::Command::new("xdg-open").arg(uri).status();
            gtk::Inhibit(true)
        });
    };

    let image = crate::scaled_image_from_resource("/org/pop/desktop-widget/extension.png", 192)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .margin_top(32)
        .build();

    Some(
        (cascade! {
            gtk::Box::new(gtk::Orientation::Vertical, 0);
            ..set_halign(gtk::Align::Center);
            ..add(header);
            ..add(&label1);
            ..add(&label_create(true, &fomat!((extensions_source.display()))));
            ..add(&cascade! {
                label_create(false, &fl!("gis-extensions-label2"));
                ..set_margin_top(12);
            });
            ..add(&label_create(true, &fomat!((extensions_backup.display()))));
            ..add(&image);
        })
        .upcast(),
    )
}

pub fn title() -> String { fl!("gis-extensions-title") }

fn gnome_shell_data_dir() -> PathBuf {
    glib::get_user_data_dir().expect("XDG_DATA_HOME path not found").join("gnome-shell")
}
