use glib::translate::{FromGlibPtrNone, ToGlibPtr};
use gtk_sys::GtkWidget;
use pop_desktop_widget::PopDesktopWidget as RustWidget;
use pop_desktop_widget::localize;
use std::ffi::CString;

#[no_mangle]
pub struct PopDesktopWidget;

#[no_mangle]
pub extern "C" fn pop_desktop_widget_new(stack: *mut gtk_sys::GtkStack) -> *mut PopDesktopWidget {
    unsafe {
        gtk::set_initialized();
    }

    localize();

    Box::into_raw(Box::new(RustWidget::new(unsafe { &gtk::Stack::from_glib_none(stack) })))
        as *mut PopDesktopWidget
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_free(widget: *mut PopDesktopWidget) {
    unsafe { Box::from_raw(widget as *mut RustWidget) };
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_dock_page(header: *mut GtkWidget) -> *mut GtkWidget {
    unsafe {
        gtk::set_initialized();
    }

    let header = unsafe { gtk::Widget::from_glib_none(header) };
    pop_desktop_widget::gis::dock::page(&header).to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_dock_title() -> *mut libc::c_char {
    string_create(pop_desktop_widget::gis::dock::title())
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_extensions_page(header: *mut GtkWidget) -> *mut GtkWidget {
    unsafe {
        gtk::set_initialized();
    }

    let header = unsafe { gtk::Widget::from_glib_none(header) };
    pop_desktop_widget::gis::extensions::page(&header).to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_extensions_title() -> *mut libc::c_char {
    string_create(pop_desktop_widget::gis::extensions::title())
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_gestures_page(header: *mut GtkWidget) -> *mut GtkWidget {
    unsafe {
        gtk::set_initialized();
    }

    let header = unsafe { gtk::Widget::from_glib_none(header) };
    pop_desktop_widget::gis::gestures::page(&header).to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_gestures_title() -> *mut libc::c_char {
    string_create(pop_desktop_widget::gis::gestures::title())
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_launcher_page(header: *mut GtkWidget) -> *mut GtkWidget {
    unsafe {
        gtk::set_initialized();
    }

    let header = unsafe { gtk::Widget::from_glib_none(header) };
    pop_desktop_widget::gis::launcher::page(&header).to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_launcher_title() -> *mut libc::c_char {
    string_create(pop_desktop_widget::gis::launcher::title())
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gis_panel_page(header: *mut GtkWidget) -> *mut GtkWidget {
    unsafe {
        gtk::set_initialized();
    }

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
pub extern "C" fn pop_desktop_widget_localize() {
    // TODO: Integrate with i18n-embed-fluent
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_string_free(string: *mut libc::c_char) {
    string_free(string)
}

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
