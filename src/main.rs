#[macro_use]
extern crate gtk_extras;

use gio::prelude::*;
use gtk::prelude::*;
use pop_desktop_widget::PopDesktopWidget;

pub const APP_ID: &str = "com.system76.PopDesktopWidget";

fn monitors() -> Result<(), Box<dyn std::error::Error>> {
    let display_manager = gdk::DisplayManager::get();
    if let Some(display) = display_manager.get_default_display() {
        for i in 0..display.get_n_monitors() {
            if let Some(monitor) = display.get_monitor(i) {
                let rect = monitor.get_geometry();
                println!("{}: {}, {}, {}, {}", i, rect.x, rect.y, rect.width, rect.height);
                if let Some(manufacturer) = monitor.get_manufacturer() {
                    println!("  Manufacturer: {}", manufacturer);
                }
                if let Some(model) = monitor.get_model() {
                    println!("  Model: {}", model);
                }
            } else {
                eprintln!("Failed to get monitor {}", i);
            }
        }
    } else {
        eprintln!("Failed to get default display");
    }

    Ok(())}

fn main() {
    glib::set_program_name(APP_ID.into());
    gtk::init().expect("failed to init GTK");

    if let Err(err) = monitors() {
        eprintln!("monitors error: {}", err);
    }

    let application = gtk::ApplicationBuilder::new().application_id(APP_ID).build();

    application.connect_activate(|app| {
        if let Some(window) = app.get_window_by_id(0) {
            window.present();
        }
    });

    application.connect_startup(|app| {
        let stack = gtk::Stack::new();
        let stack_switcher = cascade! {
            gtk::StackSwitcher::new();
            ..set_stack(Some(&stack));
        };

        PopDesktopWidget::new(&stack);

        let headerbar = gtk::HeaderBarBuilder::new()
            .custom_title(&stack_switcher)
            .show_close_button(true)
            .build();

        let _window = cascade! {
            gtk::ApplicationWindowBuilder::new()
                .application(app)
                .icon_name("pop-desktop-widget")
                .window_position(gtk::WindowPosition::Center)
                .default_height(600)
                .default_width(800)
                .build();
            ..set_titlebar(Some(&headerbar));
            ..add(&stack);
            ..show_all();
            ..connect_delete_event(move |window, _| {
                window.close();

                // Allow this closure to attain ownership of our firmware widget,
                // so that this widget will exist for as long as the window exists.
                let _widget = &stack;

                Inhibit(false)
            });
        };
    });

    application.run(&[]);
}
