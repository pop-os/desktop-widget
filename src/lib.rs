#[macro_use]
extern crate gtk_extras;

use gio::{SettingsBindFlags, SettingsExt};
use glib::clone;
use gtk::prelude::*;
use gtk_extras::settings;
use libhandy::prelude::*;
use pop_theme_switcher::PopThemeSwitcher;

use std::ops::Deref;

pub struct PopDesktopWidget(gtk::Container);

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

fn radio_bindings(settings: &gio::Settings, key: &'static str, radios: Vec<(glib::Variant, gtk::RadioButton)>) {
    // Set active radio when settings change
    //TODO: if settings is dropped, changed event fails. Would only happen if radios is empty
    {
        let radios = radios.clone();
        settings.connect_changed(move |settings, event_key| {
            if event_key == key {
                let event_value = settings.get_value(key);
                for (value, radio) in radios.iter() {
                    if &event_value == value {
                        radio.set_active(true);
                    }
                }
            }
        });
    }

    // Set active radio based on current settings
    {
        let current_value = settings.get_value(key);
        for (value, radio) in radios.iter() {
            if &current_value == value {
                radio.set_active(true);
            }
        }
    }

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
    stack.add_titled(&vbox, title, title);
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
        ..set_selection_mode(gtk::SelectionMode::None);
    };
    vbox.add(&list_box);

    list_box
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

    switch_row(&list_box, "Show Workspaces Button (TODO)");
    switch_row(&list_box, "Show Applications Button (TODO)");
    combo_row(&list_box, "Show Top Bar on Display (TODO)", "Primary Display", &[
        "Primary Display",
        "All Displays",
        "TODO"
    ]);
    combo_row(&list_box, "Date and Time Position (TODO)", "Center", &[
        "Center",
        "Left",
        "Right"
    ]);
}

fn window_controls<C: ContainerExt>(container: &C) {
    let list_box = settings_list_box(container, "Window Controls");

    switch_row(&list_box, "Show Minimize Button (TODO)");
    switch_row(&list_box, "Show Maximize Button (TODO)");
}

fn main_page(stack: &gtk::Stack) {
    let page = settings_page(stack, "Desktop");

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

    combo_row(&list_box, "Show Dock on Display (TODO)", "Primary Display", &[
        "Primary Display",
        "All Displays",
        "TODO"
    ]);

    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.dash-to-dock") {
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

        let radio_small = radio_row(&list_box, "Small", None);
        let radio_medium = radio_row(&list_box, "Medium", None);
        radio_medium.join_group(Some(&radio_small));
        let radio_large = radio_row(&list_box, "Large", None);
        radio_large.join_group(Some(&radio_small));

        radio_bindings(&settings, "dash-max-icon-size", vec![
            (glib::Variant::from(24i32), radio_small),
            (glib::Variant::from(32i32), radio_medium),
            (glib::Variant::from(48i32), radio_large),
        ]);
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
        ]);
    }
}

fn dock_page(stack: &gtk::Stack) {
    let page = settings_page(&stack, "Dock");

    let list_box = cascade! {
        gtk::ListBox::new();
        ..set_selection_mode(gtk::SelectionMode::None);
    };
    page.add(&list_box);

    switch_row(&list_box, "Show Dock on the Desktop (TODO)");

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
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 12);

        let stack_switcher = gtk::StackSwitcher::new();
        container.add(&stack_switcher);

        let stack = gtk::Stack::new();
        stack_switcher.set_stack(Some(&stack));
        container.add(&stack);

        main_page(&stack);
        appearance_page(&stack);
        dock_page(&stack);
        workspaces_page(&stack);

        Self(container.upcast())
    }

    pub fn grab_focus(&self) {
        use gtk_extras::widgets::iter_from;

        for child in iter_from::<gtk::FlowBoxChild, gtk::Container>(&*self) {
            if let Some(inner) = child.get_child() {
                let inner = inner.downcast::<gtk::Container>().unwrap();
                if let Some(radio) = iter_from::<gtk::RadioButton, _>(&inner).next() {
                    if radio.get_active() {
                        child.grab_focus();
                    }
                }
            }
        }
    }
}

impl Deref for PopDesktopWidget {
    type Target = gtk::Container;

    fn deref(&self) -> &Self::Target { &self.0 }
}
