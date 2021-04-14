#[macro_use]
extern crate gtk_extras;

use gio::{SettingsBindFlags, SettingsExt};
use glib::clone;
use gtk::prelude::*;
use gtk_extras::settings;
use libhandy::prelude::*;
use pop_theme_switcher::PopThemeSwitcher;
use std::rc::Rc;

pub struct PopDesktopWidget;

fn header_func(row: &gtk::ListBoxRow, before: Option<&gtk::ListBoxRow>) {
    if before.is_none() {
        row.set_header::<gtk::Widget>(None)
    } else if row.get_header().is_none() {
        row.set_header(Some(&cascade! {
            gtk::Separator::new(gtk::Orientation::Horizontal);
            ..show();
        }));
    }
}

fn combo_row<C: ContainerExt>(container: &C, title: &str, active: &str, values: &[&str]) -> gtk::ComboBoxText {
    let combo = cascade! {
        gtk::ComboBoxText::new();
        ..set_valign(gtk::Align::Center);
    };
    for value in values.iter() {
        combo.append(Some(value), value);
    }
    combo.set_active_id(Some(active));
    let row = cascade! {
        libhandy::ActionRow::new();
        ..set_title(Some(title));
        ..add(&combo);
    };
    container.add(&row);
    combo
}

fn radio_bindings(settings: &gio::Settings, key: &'static str, radios: Vec<(glib::Variant, gtk::RadioButton)>, custom_radio: Option<gtk::RadioButton>) {
    let update = {
        let radios = radios.clone();
        move |event_value: glib::Variant| {
            let mut custom = true;
            for (value, radio) in radios.iter() {
                if &event_value == value {
                    radio.set_active(true);
                    custom = false;
                    break;
                }
            }
            if custom {
                if let Some(ref radio) = custom_radio {
                    radio.set_active(true);
                }
            }
        }
    };

    // Set active radio based on current settings
    update(settings.get_value(key));

    // Set active radio when settings change
    //TODO: if settings is dropped, changed event fails. Would only happen if radios is empty
    settings.connect_changed(move |settings, event_key| {
        if event_key == key {
            update(settings.get_value(key));
        }
    });

    // Set settings when radios are activated
    for (value, radio) in radios {
        radio.connect_property_active_notify(clone!(@strong settings => move |radio| {
            if radio.get_active() {
                let _ = settings.set_value(key, &value);
            }
        }));
    }
}

fn radio_row<C: ContainerExt>(container: &C, title: &str, subtitle: Option<&str>) -> gtk::RadioButton {
    let radio = cascade! {
        gtk::RadioButton::new();
        ..set_valign(gtk::Align::Center);
    };
    let row = cascade! {
        libhandy::ActionRow::new();
        ..set_title(Some(title));
        ..set_subtitle(subtitle);
        ..add(&radio);
    };
    container.add(&row);
    radio
}

fn spin_row<C: ContainerExt>(container: &C, title: &str, min: f64, max: f64, step: f64) -> gtk::SpinButton {
    let spin = cascade! {
        gtk::SpinButton::with_range(min, max, step);
        ..set_valign(gtk::Align::Center);
    };
    let row = cascade! {
        libhandy::ActionRow::new();
        ..set_title(Some(title));
        ..add(&spin);
    };
    container.add(&row);
    spin
}

fn switch_row<C: ContainerExt>(container: &C, title: &str) -> gtk::Switch {
    let switch = cascade! {
        gtk::Switch::new();
        ..set_valign(gtk::Align::Center);
    };
    let row = cascade! {
        libhandy::ActionRow::new();
        ..set_title(Some(title));
        ..add(&switch);
    };
    container.add(&row);
    switch
}

fn settings_page(stack: &gtk::Stack, title: &str) -> gtk::Box {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 48);
    let clamp = cascade! {
        libhandy::Clamp::new();
        ..set_margin_top(32);
        ..set_margin_bottom(32);
        ..set_margin_start(12);
        ..set_margin_end(12);
        ..add(&vbox);
    };
    let scrolled_window = cascade! {
        gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
        ..add(&clamp);
    };
    stack.add_titled(&scrolled_window, title, title);
    vbox
}

fn settings_list_box<C: ContainerExt>(container: &C, title: &str) -> gtk::ListBox {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 12);
    container.add(&vbox);

    let label = cascade! {
        gtk::Label::new(Some(&format!("<b>{}</b>", title)));
        ..set_use_markup(true);
        ..set_xalign(0.0);
    };
    vbox.add(&label);

    let list_box = cascade! {
        gtk::ListBox::new();
        ..get_style_context().add_class("frame");
        ..set_header_func(Some(Box::new(header_func)));
        ..set_selection_mode(gtk::SelectionMode::None);
    };
    vbox.add(&list_box);

    list_box
}

fn super_key<C: ContainerExt>(container: &C) {
    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.pop-cosmic") {
        let list_box = settings_list_box(container, "Super Key Action");

        let radio_launcher = radio_row(&list_box, "Launcher", Some(
            "Pressing the Super key opens the Launcher"
        ));
        let radio_workspaces = radio_row(&list_box, "Workspaces", Some(
            "Pressing the Super key opens the Window and Workspaces Overview"
        ));
        radio_workspaces.join_group(Some(&radio_launcher));
        let radio_applications = radio_row(&list_box, "Applications", Some(
            "Pressing the Super key opens the Applications Overview"
        ));
        radio_applications.join_group(Some(&radio_launcher));

        radio_bindings(&settings, "overlay-key-action", vec![
            (glib::Variant::from("LAUNCHER"), radio_launcher),
            (glib::Variant::from("WORKSPACES"), radio_workspaces),
            (glib::Variant::from("APPLICATIONS"), radio_applications),
        ], None);
    }
}

fn hot_corner<C: ContainerExt>(container: &C) {
    let list_box = settings_list_box(container, "Hot Corner");

    let radio_disabled = radio_row(&list_box, "Disabled (TODO)", None);
    let radio_workspaces = radio_row(&list_box, "Workspaces (TODO)", Some(
        "Placing cursor in top-left corner opens the Window and Workspaces Overview"
    ));
    radio_workspaces.join_group(Some(&radio_disabled));
    let radio_applications = radio_row(&list_box, "Applications (TODO)", Some(
        "Placing cursor in top-left corner opens the Applications Overview"
    ));
    radio_applications.join_group(Some(&radio_disabled));
}

fn top_bar<C: ContainerExt>(container: &C) {
    let list_box = settings_list_box(container, "Top Bar");

    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.pop-cosmic") {
        let switch = switch_row(&list_box, "Show Workspaces Button");
        settings.bind("show-workspaces-button", &switch, "active", SettingsBindFlags::DEFAULT);
        let switch = switch_row(&list_box, "Show Applications Button");
        settings.bind("show-applications-button", &switch, "active", SettingsBindFlags::DEFAULT);
    }

    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.multi-monitors-add-on") {
        // TODO: Use `bind_with_mapping` when gtk-rs version with that is released
        let combo = combo_row(&list_box, "Show Top Bar on Display", "Primary Display", &[
            "Primary Display",
            "All Displays",
            "TODO"
        ]);
        let id = if settings.get_boolean("show-panel") {
            "All Displays"
        } else {
            "Primary Display"
        };
        combo.set_active_id(Some(id));
        combo.connect_changed(clone!(@strong settings => move |combo| {
            let all_displays = combo.get_active_id().map_or(false, |x| x == "All Displays" );
            settings.set_boolean("show-panel", all_displays).unwrap();
        }));
    }

    combo_row(&list_box, "Date and Time Position (TODO)", "Center", &[
        "Center",
        "Left",
        "Right"
    ]);
}

pub struct ButtonLayout {
    settings: gio::Settings,
    key: &'static str,
    switch_min: gtk::Switch,
    switch_max: gtk::Switch,
}

impl ButtonLayout {
    fn connect(self: Rc<Self>) {
        self.update(false);

        let self_event = self.clone();
        self.settings.connect_changed(move |_, event_key| {
            if event_key == self_event.key {
                self_event.update(false);
            }
        });

        let self_event = self.clone();
        self.switch_min.connect_property_active_notify(move |_| {
            self_event.update(true);
        });

        let self_event = self.clone();
        self.switch_max.connect_property_active_notify(move |_| {
            self_event.update(true);
        });
    }

    fn update(&self, write: bool) {
        let default_value = "appmenu:close";
        let value = self.settings.get_string("button-layout")
            .unwrap_or(glib::GString::from(default_value));
        if write {
            let new_value = match (self.switch_min.get_active(), self.switch_max.get_active()) {
                (false, false) => default_value,
                (false, true) => "appmenu:maximize,close",
                (true, false) => "appmenu:minimize,close",
                (true, true) => "appmenu:minimize,maximize,close",
            };

            let _ = self.settings.set_string("button-layout", &new_value);
        } else {
            self.switch_min.set_active(value.contains("minimize"));
            self.switch_max.set_active(value.contains("maximize"));
        }
    }
}

fn window_controls<C: ContainerExt>(container: &C) {
    if let Some(settings) = settings::new_checked("org.gnome.desktop.wm.preferences") {
        let list_box = settings_list_box(container, "Window Controls");

        let switch_min = switch_row(&list_box, "Show Minimize Button");
        let switch_max = switch_row(&list_box, "Show Maximize Button");

        let button_layout = Rc::new(ButtonLayout {
            settings,
            key: "button-layout",
            switch_min,
            switch_max,
        });

        button_layout.connect();
    }
}

fn main_page(stack: &gtk::Stack) {
    let page = settings_page(stack, "Desktop");

    super_key(&page);
    hot_corner(&page);
    top_bar(&page);
    window_controls(&page);
}

fn appearance_page(stack: &gtk::Stack) {
    let page = settings_page(&stack, "Appearance");

    let theme_switcher = PopThemeSwitcher::new();
    page.add(&*theme_switcher);
}

fn dock_options<C: ContainerExt>(container: &C) {
    let list_box = settings_list_box(container, "Dock Options");

    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.dash-to-dock") {
        // TODO: Use `bind_with_mapping` when gtk-rs version with that is released
        let combo = combo_row(&list_box, "Show Dock on Display", "Primary Display", &[
            "Primary Display",
            "All Displays",
            "TODO"
        ]);
        let id = if settings.get_boolean("multi-monitor") {
            "All Displays"
        } else {
            "Primary Display"
        };
        combo.set_active_id(Some(id));
        combo.connect_changed(clone!(@strong settings => move |combo| {
            let all_displays = combo.get_active_id().map_or(false, |x| x == "All Displays" );
            settings.set_boolean("multi-monitor", all_displays).unwrap();
        }));

        let switch = switch_row(&list_box, "Automatically Hide Dock");
        settings.bind("dock-fixed", &switch, "active", SettingsBindFlags::DEFAULT | SettingsBindFlags::INVERT_BOOLEAN);

        let switch = switch_row(&list_box, "Extend dock to the edges of the screen");
        settings.bind("extend-height", &switch, "active", SettingsBindFlags::DEFAULT);
    }

    switch_row(&list_box, "Show Launcher Icon in Dock (TODO)");
    switch_row(&list_box, "Show Applications Icon in Dock (TODO)");
    switch_row(&list_box, "Show Workspaces Icon in Dock (TODO)");
}

fn dock_size<C: ContainerExt>(container: &C) {
    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.dash-to-dock") {
        let list_box = settings_list_box(container, "Dock Size");

        let radio_small = radio_row(&list_box, "Small (36px)", None);
        let radio_medium = radio_row(&list_box, "Medium (48px)", None);
        radio_medium.join_group(Some(&radio_small));
        let radio_large = radio_row(&list_box, "Large (60px)", None);
        radio_large.join_group(Some(&radio_small));
        let radio_custom = radio_row(&list_box, "Custom", None);
        radio_custom.join_group(Some(&radio_small));

        let spin = spin_row(&list_box, "Custom Size", 8.0, 128.0, 1.0);
        settings.bind("dash-max-icon-size", &spin, "value", SettingsBindFlags::DEFAULT);
        radio_custom.bind_property("active", &spin, "sensitive")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();

        radio_bindings(&settings, "dash-max-icon-size", vec![
            (glib::Variant::from(36i32), radio_small),
            (glib::Variant::from(48i32), radio_medium),
            (glib::Variant::from(60i32), radio_large),
        ], Some(radio_custom));
    }
}

fn dock_position<C: ContainerExt>(container: &C) {
    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.dash-to-dock") {
        let list_box = settings_list_box(container, "Position on the Desktop");

        let radio_bottom = radio_row(&list_box, "Bottom of the screen", None);
        let radio_left = radio_row(&list_box, "Along the left side", None);
        radio_left.join_group(Some(&radio_bottom));
        let radio_right = radio_row(&list_box, "Along the right side", None);
        radio_right.join_group(Some(&radio_bottom));

        radio_bindings(&settings, "dock-position", vec![
            (glib::Variant::from("BOTTOM"), radio_bottom),
            (glib::Variant::from("LEFT"), radio_left),
            (glib::Variant::from("RIGHT"), radio_right),
        ], None);
    }
}

fn dock_page(stack: &gtk::Stack) {
    let page = settings_page(&stack, "Dock");

    let list_box = cascade! {
        gtk::ListBox::new();
        ..get_style_context().add_class("frame");
        ..set_header_func(Some(Box::new(header_func)));
        ..set_selection_mode(gtk::SelectionMode::None);
    };
    page.add(&list_box);

    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.dash-to-dock") {
        let switch = switch_row(&list_box, "Show Dock");
        settings.bind("manualhide", &switch, "active", SettingsBindFlags::DEFAULT | SettingsBindFlags::INVERT_BOOLEAN);
    }

    dock_options(&page);
    dock_size(&page);
    dock_position(&page);
}

fn workspaces_multi_monitor<C: ContainerExt>(container: &C) {
    let list_box = settings_list_box(container, "Multi-monitor Behavior");

    let radio_span = radio_row(&list_box, "Workspaces Span Displays (TODO)", None);
    let radio_separate = radio_row(&list_box, "Displays Have Separate Workspaces (TODO)", None);
    radio_separate.join_group(Some(&radio_span));
}

fn workspaces_position<C: ContainerExt>(container: &C) {
    let list_box = settings_list_box(container, "Placement of the Workspace Picker");

    let radio_left = radio_row(&list_box, "Along the Left Side (TODO)", None);
    let radio_right = radio_row(&list_box, "Along the Right Side (TODO)", None);
    radio_right.join_group(Some(&radio_left));
    let radio_top = radio_row(&list_box, "Top of the Screen (TODO)", None);
    radio_top.join_group(Some(&radio_left));
    let radio_bottom = radio_row(&list_box, "Bottom of the Screen (TODO)", None);
    radio_bottom.join_group(Some(&radio_left));
}

fn workspaces_page(stack: &gtk::Stack) {

    let page = settings_page(&stack, "Workspaces");

    let list_box = cascade! {
        gtk::ListBox::new();
        ..get_style_context().add_class("frame");
        ..set_header_func(Some(Box::new(header_func)));
        ..set_selection_mode(gtk::SelectionMode::None);
    };
    page.add(&list_box);

    if let Some(settings) = settings::new_checked("org.gnome.mutter") {
        let radio_dynamic = radio_row(&list_box, "Dynamic Workspaces", Some(
            "Automatically removes empty workspaces."
        ));
        settings.bind("dynamic-workspaces", &radio_dynamic, "active", SettingsBindFlags::DEFAULT);
        let radio_fixed = radio_row(&list_box, "Fixed Number of Workspaces", Some(
            "Specify a number of workspaces"
        ));
        radio_fixed.join_group(Some(&radio_dynamic));

        if let Some(settings) = settings::new_checked("org.gnome.desktop.wm.preferences") {
            let spin_number = spin_row(&list_box, "Number of Workspaces", 1.0, 36.0, 1.0);
            settings.bind("num-workspaces", &spin_number, "value", SettingsBindFlags::DEFAULT);
            radio_fixed.bind_property("active", &spin_number, "sensitive")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();
        }
    }

    workspaces_multi_monitor(&page);
    workspaces_position(&page);
}

impl PopDesktopWidget {
    pub fn new(stack: &gtk::Stack) -> Self {
        main_page(&stack);
        appearance_page(&stack);
        dock_page(&stack);
        workspaces_page(&stack);

        stack.show_all();

        Self
    }
}
