use glib::{
    translate::{FromGlibPtrNone, ToGlibPtr},
    Cast,
};
use gtk::prelude::*;
use gtk_sys::GtkWidget;
use pop_desktop_widget::{localize, PopDesktopWidget as RustWidget};
use std::ffi::CString;

pub struct PopDesktopWidget;

#[no_mangle]
pub extern "C" fn pop_desktop_widget_new(stack: *mut gtk_sys::GtkStack) -> *mut PopDesktopWidget {
    initialize();

    Box::into_raw(Box::new(RustWidget::new(unsafe { &gtk::Stack::from_glib_none(stack) })))
        as *mut PopDesktopWidget
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_free(widget: *mut PopDesktopWidget) {
    unsafe { Box::from_raw(widget as *mut RustWidget) };
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_dock_page(header: *mut GtkWidget) -> *mut GtkWidget {
    initialize();

    let header = unsafe { gtk::Widget::from_glib_none(header) };
    pop_desktop_widget::gis::dock::page(&header).to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_dock_title() -> *mut libc::c_char {
    string_create(pop_desktop_widget::gis::dock::title())
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_extensions_page(header: *mut GtkWidget) -> *mut GtkWidget {
    initialize();

    let header = unsafe { gtk::Widget::from_glib_none(header) };
    pop_desktop_widget::gis::extensions::page(&header).to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_extensions_title() -> *mut libc::c_char {
    string_create(pop_desktop_widget::gis::extensions::title())
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_gestures_page(header: *mut GtkWidget) -> *mut GtkWidget {
    initialize();

    let header = unsafe { gtk::Widget::from_glib_none(header) };
    pop_desktop_widget::gis::gestures::page(&header).to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_gestures_title() -> *mut libc::c_char {
    string_create(pop_desktop_widget::gis::gestures::title())
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_launcher_page(header: *mut GtkWidget) -> *mut GtkWidget {
    initialize();

    let header = unsafe { gtk::Widget::from_glib_none(header) };
    pop_desktop_widget::gis::launcher::page(&header).to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_launcher_title() -> *mut libc::c_char {
    string_create(pop_desktop_widget::gis::launcher::title())
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_panel_page(header: *mut GtkWidget) -> *mut GtkWidget {
    initialize();

    let header = unsafe { gtk::Widget::from_glib_none(header) };
    pop_desktop_widget::gis::panel::page(&header).to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_panel_title() -> *mut libc::c_char {
    string_create(pop_desktop_widget::gis::panel::title())
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gresource_init() {
    pop_desktop_widget::gresource::init().expect("failed to load gresource for pop-desktop-widget");
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_localize() { pop_desktop_widget::localize(); }

#[no_mangle]
pub extern "C" fn pop_desktop_widget_string_free(string: *mut libc::c_char) { string_free(string) }

fn string_create(string: String) -> *mut libc::c_char {
    CString::new(string).expect("Rust string contained null").into_raw()
}

fn string_free(string: *mut libc::c_char) {
    if !string.is_null() {
        unsafe {
            CString::from_raw(string);
        }
    }
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gcc_main_page() -> *mut GtkWidget {
    initialize();
    let widget = pop_desktop_widget::main_page();
    widget.show_all();
    widget.upcast::<gtk::Widget>().to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gcc_appearance_page() -> *mut GtkWidget {
    initialize();
    let widget = pop_desktop_widget::appearance_page();
    widget.show_all();
    widget.upcast::<gtk::Widget>().to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gcc_dock_page() -> *mut GtkWidget {
    initialize();
    let widget = pop_desktop_widget::dock_page();
    widget.show_all();
    widget.upcast::<gtk::Widget>().to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gcc_workspaces_page() -> *mut GtkWidget {
    initialize();
    let widget = pop_desktop_widget::workspaces_page();
    widget.show_all();
    widget.upcast::<gtk::Widget>().to_glib_full()
}

fn initialize() {
    unsafe {
        gtk::set_initialized();
    }

    localize();
}
