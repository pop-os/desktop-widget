#[macro_use]
extern crate fomat_macros;
#[macro_use]
extern crate gtk_extras;

pub mod gis;
pub mod gresource;
mod gst_video;
pub mod localize;

use gio::{Settings, SettingsBindFlags, SettingsExt};
use glib::clone;
use gtk::prelude::*;
use gtk_extras::settings;
use i18n_embed::DesktopLanguageRequester;
use libhandy::prelude::*;
use pop_theme_switcher::PopThemeSwitcher;
use std::{cell::RefCell, rc::Rc};

const PAGE_APPEARANCE: &str = "appearance";
const PAGE_DOCK: &str = "dock";
const PAGE_MAIN: &str = "main";
const PAGE_WORKSPACES: &str = "workspaces";

pub fn localize() {
    let localizer = crate::localize::localizer();
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Err(error) = localizer.select(&requested_languages) {
        eprintln!("Error while loading language for pop-desktop-widget {}", error);
    }
}

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

fn combo_row<C: ContainerExt>(
    container: &C,
    title: &str,
    active: &str,
    values: &[&str],
) -> gtk::ComboBoxText {
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

fn radio_bindings(
    settings: &gio::Settings,
    key: &'static str,
    radios: Vec<(glib::Variant, gtk::RadioButton)>,
    custom_radio: Option<gtk::RadioButton>,
) {
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
    // TODO: if settings is dropped, changed event fails. Would only happen if radios is empty
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

fn radio_row<C: ContainerExt>(
    container: &C,
    title: &str,
    subtitle: Option<&str>,
) -> gtk::RadioButton {
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

fn scaled_image_from_resource(resource: &str, pixels: i32) -> gtk::ImageBuilder {
    let pixels = f64::from(pixels);
    let mut pixbuf = gdk_pixbuf::Pixbuf::from_resource(resource).expect("missing resource");

    let mut width = f64::from(pixbuf.get_width());
    let mut height = f64::from(pixbuf.get_height());
    let scale = f64::min(pixels / width, pixels / height);

    width = scale * width;
    height = scale * height;

    pixbuf = pixbuf
        .scale_simple(width.round() as i32, height.round() as i32, gdk_pixbuf::InterpType::Hyper)
        .unwrap();

    gtk::ImageBuilder::new().pixbuf(&pixbuf)
}

fn spin_row<C: ContainerExt>(
    container: &C,
    title: &str,
    min: f64,
    max: f64,
    step: f64,
) -> gtk::SpinButton {
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

/// Template for a settings page. `id` is used internally for stack switching.
fn settings_page(stack: &gtk::Stack, id: &str, title: &str) -> gtk::Box {
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
    stack.add_titled(&scrolled_window, id, title);
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
        let list_box = settings_list_box(container, &fl!("super-key-action"));

        let radio_launcher = radio_row(
            &list_box,
            &fl!("action-launcher"),
            Some(&fl!("action-launcher-description")),
        );
        let radio_workspaces = radio_row(
            &list_box,
            &fl!("action-workspaces"),
            Some(&fl!("action-workspaces-description")),
        );
        radio_workspaces.join_group(Some(&radio_launcher));
        let radio_applications = radio_row(
            &list_box,
            &fl!("action-applications"),
            Some(&fl!("action-applications-description")),
        );
        radio_applications.join_group(Some(&radio_launcher));

        radio_bindings(
            &settings,
            "overlay-key-action",
            vec![
                (glib::Variant::from("LAUNCHER"), radio_launcher),
                (glib::Variant::from("WORKSPACES"), radio_workspaces),
                (glib::Variant::from("APPLICATIONS"), radio_applications),
            ],
            None,
        );
    }
}

fn hot_corner<C: ContainerExt>(container: &C) {
    // TODO: Support more options in the future

    let list_box = settings_list_box(container, &fl!("hot-corner"));
    let settings = gio::Settings::new("org.gnome.desktop.interface");

    let switch = switch_row(&list_box, &fl!("hot-corner-description"));
    settings.bind("enable-hot-corners", &switch, "active", SettingsBindFlags::DEFAULT);
}

fn top_bar<C: ContainerExt>(container: &C) {
    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.pop-cosmic") {
        let switch = switch_row(container, &fl!("show-workspaces-button"));
        settings.bind("show-workspaces-button", &switch, "active", SettingsBindFlags::DEFAULT);

        let switch = switch_row(container, &fl!("show-applications-button"));
        settings.bind("show-applications-button", &switch, "active", SettingsBindFlags::DEFAULT);

        let center = &fl!("date-combo-center");

        cascade! {
            combo_row(container, &fl!("date-combo"), center, &[
                center,
                &fl!("date-combo-left"),
                &fl!("date-combo-right")
            ]);
            ..set_active(Some(settings.get_enum("clock-alignment") as u32));
            ..connect_changed(clone!(@strong settings => move |combo| {
                settings.set_enum("clock-alignment", combo.get_active().unwrap_or(0) as i32).unwrap();
            }));
        };
    }
}

pub struct ButtonLayout {
    settings:   gio::Settings,
    key:        &'static str,
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
        let value =
            self.settings.get_string("button-layout").unwrap_or(glib::GString::from(default_value));
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
        let list_box = settings_list_box(container, &fl!("window-controls"));

        let switch_min = switch_row(&list_box, &fl!("show-minimize-button"));
        let switch_max = switch_row(&list_box, &fl!("show-maximize-button"));

        let button_layout =
            Rc::new(ButtonLayout { settings, key: "button-layout", switch_min, switch_max });

        button_layout.connect();
    }
}

fn main_page(stack: &gtk::Stack) {
    let page = settings_page(stack, PAGE_MAIN, &fl!("page-main"));

    super_key(&page);
    hot_corner(&page);
    top_bar(&settings_list_box(&page, &fl!("top-bar")));
    window_controls(&page);
}

fn appearance_page(stack: &gtk::Stack) {
    let page = settings_page(&stack, PAGE_APPEARANCE, &fl!("page-appearance"));

    let theme_switcher = PopThemeSwitcher::new();
    page.add(&*theme_switcher);
}

fn dock_options<C: ContainerExt>(container: &C) {
    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.dash-to-dock") {
        let list_box = settings_list_box(container, &fl!("dock-options"));

        let switch = switch_row(&list_box, &fl!("dock-extend"));
        settings.bind("extend-height", &switch, "active", SettingsBindFlags::DEFAULT);

        let shell_settings = gio::Settings::new("org.gnome.shell");
        let launcher_switch = switch_row(&list_box, &fl!("dock-launcher"));
        let workspaces_switch = switch_row(&list_box, &fl!("dock-workspaces"));
        let applications_switch = switch_row(&list_box, &fl!("dock-applications"));
        let update_switches = clone!(@strong shell_settings, @strong launcher_switch, @strong workspaces_switch, @strong applications_switch => move || {
            let mut launcher_active = false;
            let mut workspaces_active = false;
            let mut applications_active = false;
            for favorite in shell_settings.get_strv("favorite-apps") {
                match favorite.as_str() {
                    "pop-cosmic-launcher.desktop" => launcher_active = true,
                    "pop-cosmic-workspaces.desktop" => workspaces_active = true,
                    "pop-cosmic-applications.desktop" => applications_active = true,
                    _ => {}
                }
            }
            launcher_switch.set_active(launcher_active);
            workspaces_switch.set_active(workspaces_active);
            applications_switch.set_active(applications_active);
        });
        update_switches();
        let switch_handlers =
            Rc::new(RefCell::new(Vec::<(gtk::Switch, glib::SignalHandlerId)>::new()));
        let handler_id = Rc::new(
            shell_settings
                .connect_local(
                    "changed::favorite-apps",
                    false,
                    clone!(@strong switch_handlers => move |_| {
                        let ids = switch_handlers.borrow();
                        for (switch, id) in ids.iter() {
                            switch.block_signal(id);
                        }
                        update_switches();
                        for (switch, id) in ids.iter() {
                            switch.unblock_signal(id);
                        }
                        None
                    }),
                )
                .unwrap(),
        );
        let connect_switch = move |switch: &gtk::Switch, desktop, pos: usize| {
            let shell_settings = shell_settings.clone();
            let handler_id = handler_id.clone();
            let id = switch.connect_property_active_notify(move |switch| {
                let active = switch.get_active();
                let favorites = shell_settings.get_strv("favorite-apps");
                let mut favorites = favorites.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                let index = favorites.iter().position(|x| *x == desktop);
                if !active {
                    if let Some(index) = index {
                        favorites.remove(index);
                    }
                } else if index.is_none() {
                    // Insert at `pos`, or before first non-cosmic favorite
                    let pos = pos.min(
                        favorites
                            .iter()
                            .position(|x| {
                                ![
                                    "pop-cosmic-launcher.desktop",
                                    "pop-cosmic-workspaces.desktop",
                                    "pop-cosmic-applications.desktop",
                                ]
                                .contains(x)
                            })
                            .unwrap_or(0),
                    );
                    favorites.insert(pos, desktop);
                }
                shell_settings.block_signal(&handler_id);
                shell_settings.set_strv("favorite-apps", &favorites).unwrap();
                shell_settings.unblock_signal(&handler_id);
            });
            switch_handlers.borrow_mut().push((switch.clone(), id));
        };
        connect_switch(&launcher_switch, "pop-cosmic-launcher.desktop", 0);
        connect_switch(&workspaces_switch, "pop-cosmic-workspaces.desktop", 1);
        connect_switch(&applications_switch, "pop-cosmic-applications.desktop", 2);

        let switch = switch_row(&list_box, &fl!("dock-mounted-drives"));
        settings.bind("show-mounts", &switch, "active", SettingsBindFlags::DEFAULT);
        
        fn map_click_action_selection(selection: i32) -> &'static str {
            return match selection {
                0 => "cycle-windows",
                1 => "minimize",
                2 => "minimize-or-previews",
                _ => "cycle-windows"
            };
        }
        fn map_click_action_setting(setting: &str) -> u32 {
            return match setting {
                "cycle-windows" => 0,
                "minimize" => 1,
                "minimize-or-previews" => 2,
                _ => 0
            }
        }
        let cycle_windows = &fl!("click-action-cycle");
        let minimize = &fl!("click-action-minimize");
        let minimize_or_previews = &fl!("click-action-minimize-or-previews");
        cascade! {
            combo_row(&list_box, &fl!("dock-click-action"), cycle_windows, &[
                cycle_windows,
                minimize,
                minimize_or_previews
            ]);
            ..set_active(Some(map_click_action_setting(&settings.get_string("click-action").unwrap())));
            ..connect_changed(clone!(@strong settings => move |combo| {
                let click_action_selection = combo.get_active().unwrap_or(0) as i32;
                settings.set_string("click-action", map_click_action_selection(click_action_selection)).unwrap();
            }));
        };
    }
}

fn dock_selector() -> gtk::Box {
    let container = cascade! {
        gtk::Box::new(gtk::Orientation::Horizontal, 12);
        ..set_vexpand(true);
        ..set_valign(gtk::Align::Center);
        ..set_homogeneous(true);
    };

    if let Some(settings) =
        gtk_extras::settings::new_checked("org.gnome.shell.extensions.dash-to-dock")
    {
        let radio_no_dock = gtk::RadioButton::with_label(&fl!("dock-disable"));
        settings.bind("manualhide", &radio_no_dock, "active", gio::SettingsBindFlags::DEFAULT);

        let radio_extend =
            gtk::RadioButton::with_label_from_widget(&radio_no_dock, &fl!("dock-extends"));
        settings.bind("extend-height", &radio_extend, "active", gio::SettingsBindFlags::DEFAULT);

        let radio_no_extend =
            gtk::RadioButton::with_label_from_widget(&radio_extend, &fl!("dock-dynamic"));

        (if settings.get_boolean("manualhide") {
            &radio_no_dock
        } else if settings.get_boolean("extend-height") {
            &radio_extend
        } else {
            &radio_no_extend
        })
        .set_active(true);

        fn create_option(button: &gtk::RadioButton, image_resource: &str) -> gtk::Box {
            let image = gtk::ImageBuilder::new()
                .resource(image_resource)
                .halign(gtk::Align::Start)
                .margin_start(4)
                .build();

            cascade! {
                gtk::Box::new(gtk::Orientation::Vertical, 16);
                ..add(button);
                ..add(&image);
            }
        }

        container.add(&create_option(&radio_no_dock, "/org/pop/desktop-widget/no-dock.png"));
        container.add(&create_option(&radio_extend, "/org/pop/desktop-widget/extend.png"));
        container.add(&create_option(&radio_no_extend, "/org/pop/desktop-widget/no-extend.png"));
    }

    container
}

fn dock_visibility<C: ContainerExt>(container: &C) {
    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.dash-to-dock") {
        let list_box = settings_list_box(container, &fl!("dock-visibility"));

        let radio_visible = radio_row(&list_box, &fl!("dock-always-visible"), None);
        let radio_autohide = radio_row(
            &list_box,
            &fl!("dock-always-hide"),
            Some(&fl!("dock-always-hide-description")),
        );
        radio_autohide.join_group(Some(&radio_visible));
        let radio_intellihide = radio_row(
            &list_box,
            &fl!("dock-intelligently-hide"),
            Some(&fl!("dock-intelligently-hide-description")),
        );
        radio_intellihide.join_group(Some(&radio_visible));

        let update_radios = clone!(@strong radio_visible, @strong radio_autohide, @strong radio_intellihide => move |settings: &gio::Settings| {
            let radio = if settings.get_boolean("dock-fixed") {
                &radio_visible
            } else if settings.get_boolean("intellihide") {
                &radio_intellihide
            } else {
                &radio_autohide
            };
            if !radio.get_active() {
                radio.set_active(true);
            }
        });
        update_radios(&settings);
        // shell_settings.block_signal(&handler_id);
        let handler_id = Rc::new(settings.connect_changed(move |settings, key| {
            if key == "dock-fixed" || key == "intellihide" {
                update_radios(settings);
            }
        }));
        radio_visible.connect_property_active_notify(
            clone!(@strong settings, @strong handler_id => move |radio| {
                if !radio.get_active() {
                    return;
                }
                settings.block_signal(&handler_id);
                settings.set_boolean("dock-fixed", true).unwrap();
                settings.set_boolean("intellihide", false).unwrap();
                settings.unblock_signal(&handler_id);
            }),
        );
        radio_intellihide.connect_property_active_notify(
            clone!(@strong settings, @strong handler_id => move |radio| {
                if !radio.get_active() {
                    return;
                }
                settings.block_signal(&handler_id);
                settings.set_boolean("dock-fixed", false).unwrap();
                settings.set_boolean("intellihide", true).unwrap();
                settings.unblock_signal(&handler_id);
            }),
        );
        radio_autohide.connect_property_active_notify(clone!(@strong settings => move |radio| {
            if !radio.get_active() {
                return;
            }
            settings.block_signal(&handler_id);
            settings.set_boolean("dock-fixed", false).unwrap();
            settings.set_boolean("intellihide", false).unwrap();
            settings.unblock_signal(&handler_id);
        }));

        // TODO: Use `bind_with_mapping` when gtk-rs version with that is released
        let primary = fl!("display-primary");
        let all = fl!("display-all");
        let combo = combo_row(&list_box, &fl!("dock-show-on-display"), &primary, &[&primary, &all]);
        let id = if settings.get_boolean("multi-monitor") { &all } else { &primary };
        combo.set_active_id(Some(id));
        combo.connect_changed(clone!(@strong settings => move |combo| {
            let all_displays = combo.get_active_id().map_or(false, |x| &x == &all );
            settings.set_boolean("multi-monitor", all_displays).unwrap();
        }));
    }
}

fn dock_size<C: ContainerExt>(container: &C) {
    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.dash-to-dock") {
        let list_box = settings_list_box(container, "Dock Size");

        let mut description: String = [&fl!("size-small"), " (36px)"].concat();
        let radio_small = radio_row(&list_box, &description, None);

        description = [&fl!("size-medium"), " (48px)"].concat();
        let radio_medium = radio_row(&list_box, &description, None);
        radio_medium.join_group(Some(&radio_small));

        description = [&fl!("size-large"), " (60px)"].concat();
        let radio_large = radio_row(&list_box, &description, None);
        radio_large.join_group(Some(&radio_small));

        let radio_custom = gtk::RadioButton::new();
        radio_custom.set_no_show_all(true);
        radio_custom.join_group(Some(&radio_small));

        let spin = spin_row(&list_box, &fl!("size-custom"), 8.0, 128.0, 1.0);
        settings.bind("dash-max-icon-size", &spin, "value", SettingsBindFlags::DEFAULT);

        radio_bindings(
            &settings,
            "dash-max-icon-size",
            vec![
                (glib::Variant::from(36i32), radio_small),
                (glib::Variant::from(48i32), radio_medium),
                (glib::Variant::from(60i32), radio_large),
            ],
            Some(radio_custom),
        );
    }
}

fn dock_position<C: ContainerExt>(container: &C) {
    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.dash-to-dock") {
        let list_box = settings_list_box(container, &fl!("dock-position"));

        let radio_bottom = radio_row(&list_box, &fl!("position-bottom"), None);
        let radio_left = radio_row(&list_box, &fl!("position-left"), None);
        radio_left.join_group(Some(&radio_bottom));
        let radio_right = radio_row(&list_box, &fl!("position-right"), None);
        radio_right.join_group(Some(&radio_bottom));

        radio_bindings(
            &settings,
            "dock-position",
            vec![
                (glib::Variant::from("BOTTOM"), radio_bottom),
                (glib::Variant::from("LEFT"), radio_left),
                (glib::Variant::from("RIGHT"), radio_right),
            ],
            None,
        );
    }
}

fn dock_page(stack: &gtk::Stack) {
    let page = settings_page(&stack, PAGE_DOCK, &fl!("page-dock"));

    let list_box = framed_list_box();
    page.add(&list_box);

    let switch = switch_row(&list_box, &fl!("dock-enable"));

    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.dash-to-dock") {
        settings.bind(
            "manualhide",
            &switch,
            "active",
            SettingsBindFlags::DEFAULT | SettingsBindFlags::INVERT_BOOLEAN,
        );
    }

    dock_options(&page);
    dock_visibility(&page);
    dock_size(&page);
    dock_position(&page);

    page.foreach(|child| {
        if child != list_box.upcast_ref::<gtk::Widget>() {
            switch.bind_property("active", child, "sensitive").build();
        }
    });
}

fn framed_list_box() -> gtk::ListBox {
    cascade! {
        gtk::ListBox::new();
        ..get_style_context().add_class("frame");
        ..set_header_func(Some(Box::new(header_func)));
        ..set_selection_mode(gtk::SelectionMode::None);
    }
}

fn workspaces_multi_monitor<C: ContainerExt>(container: &C) {
    let list_box = settings_list_box(container, &fl!("multi-monitor-behavior"));

    let settings = Settings::new("org.gnome.mutter");

    let radio_span = radio_row(&list_box, &fl!("workspaces-span-displays"), None);
    let radio_primary = radio_row(&list_box, &fl!("workspaces-primary"), None);
    radio_primary.join_group(Some(&radio_span));

    radio_bindings(
        &settings,
        "workspaces-only-on-primary",
        vec![(glib::Variant::from(false), radio_span), (glib::Variant::from(true), radio_primary)],
        None,
    );
}

fn workspaces_position<C: ContainerExt>(container: &C) {
    if let Some(settings) = settings::new_checked("org.gnome.shell.extensions.pop-cosmic") {
        if !settings
            .get_property_settings_schema()
            .map_or(false, |x| x.has_key("workspace-picker-left"))
        {
            return;
        }

        let list_box = settings_list_box(container, &fl!("workspace-picker-position"));

        let radio_left = radio_row(&list_box, &fl!("position-left"), None);
        let radio_right = radio_row(&list_box, &fl!("position-right"), None);
        radio_right.join_group(Some(&radio_left));
        radio_bindings(
            &settings,
            "workspace-picker-left",
            vec![
                (glib::Variant::from(false), radio_right),
                (glib::Variant::from(true), radio_left),
            ],
            None,
        );

        if let Some(mm_settings) =
            settings::new_checked("org.gnome.shell.extensions.multi-monitors-add-on")
        {
            if mm_settings
                .get_property_settings_schema()
                .map_or(false, |x| x.has_key("thumbnails-on-left-side"))
            {
                let settings_clone = settings.clone();
                settings
                    .connect_local("changed::workspace-picker-left", false, move |_| {
                        mm_settings
                            .set_boolean(
                                "thumbnails-on-left-side",
                                settings_clone.get_boolean("workspace-picker-left"),
                            )
                            .unwrap();
                        None
                    })
                    .unwrap();
            }
        }
    }
}

fn workspaces_page(stack: &gtk::Stack) {
    let page = settings_page(&stack, PAGE_WORKSPACES, &fl!("page-workspaces"));

    let list_box = cascade! {
        gtk::ListBox::new();
        ..get_style_context().add_class("frame");
        ..set_header_func(Some(Box::new(header_func)));
        ..set_selection_mode(gtk::SelectionMode::None);
    };
    page.add(&list_box);

    if let Some(settings) = settings::new_checked("org.gnome.mutter") {
        let radio_dynamic = radio_row(
            &list_box,
            &fl!("workspaces-dynamic"),
            Some(&fl!("workspaces-dynamic-description")),
        );

        let radio_fixed = radio_row(
            &list_box,
            &fl!("workspaces-fixed"),
            Some(&fl!("workspaces-fixed-description")),
        );

        radio_fixed.join_group(Some(&radio_dynamic));
        settings.bind("dynamic-workspaces", &radio_dynamic, "active", SettingsBindFlags::DEFAULT);
        settings.bind(
            "dynamic-workspaces",
            &radio_fixed,
            "active",
            SettingsBindFlags::DEFAULT | SettingsBindFlags::INVERT_BOOLEAN,
        );

        if let Some(settings) = settings::new_checked("org.gnome.desktop.wm.preferences") {
            let spin_number = spin_row(&list_box, &fl!("workspaces-amount"), 1.0, 36.0, 1.0);
            settings.bind("num-workspaces", &spin_number, "value", SettingsBindFlags::DEFAULT);
            radio_fixed
                .bind_property("active", &spin_number, "sensitive")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();
        }
    }

    workspaces_multi_monitor(&page);
    workspaces_position(&page);
}

impl PopDesktopWidget {
    pub fn new(stack: &gtk::Stack) -> Self {
        let mut children = Vec::new();
        stack.foreach(|w| {
            let name = stack.get_child_name(w).unwrap();
            let title = stack.get_child_title(w).unwrap();
            stack.remove(w);
            children.push((w.clone(), name, title));
        });

        main_page(&stack);
        for (w, name, title) in children {
            stack.add_titled(&w, &name, &title);
        }

        appearance_page(&stack);
        dock_page(&stack);
        workspaces_page(&stack);

        stack.show_all();

        if let Ok(page) = std::env::var("POP_DESKTOP_PAGE") {
            stack.set_visible_child_name(&page);
        }

        Self
    }
}
