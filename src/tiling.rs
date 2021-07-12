use gio::prelude::*;
use gtk::prelude::*;
use gtk_extras::settings;
use gio::SettingsBindFlags;
use crate::fl;

pub fn view(stack: &gtk::Stack) {
    let page = crate::settings_page(stack, "tiling", &fl!("page-tiling"));

    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.pop-shell") {
        let toggle_switch = |list: &gtk::ListBox, description: &str, property: &str| {
            let switch = crate::switch_row(list, description);
            settings.bind(property, &switch, "active", SettingsBindFlags::DEFAULT);
        };

        let spin_button = |list: &gtk::ListBox, description: &str, property: &str| {
            let spin = crate::spin_row(list, description, 0.0, 64.0, 1.0);
            settings.bind(property, &spin, "value", SettingsBindFlags::DEFAULT);
        };

        let list = crate::framed_list_box();

        toggle_switch(&list, &fl!("tiling-show-title"), "show-title");
        toggle_switch(&list, &fl!("tiling-active-hint"), "active-hint");
        toggle_switch(&list, &fl!("tiling-snap-to-grid"), "snap-to-grid");

        page.add(&list);

        let list = crate::settings_list_box(&page, &fl!("tiling-gaps"));

        spin_button(&list, &fl!("tiling-gap-outer"), "gap-outer");
        spin_button(&list, &fl!("tiling-gap-inner"), "gap-inner");
        toggle_switch(&list, &fl!("tiling-smart-gaps"), "smart-gaps");
    }
}