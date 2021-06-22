use glib::translate::{FromGlibPtrNone, ToGlibPtr};
use gtk_sys::GtkWidget;
use pop_desktop_widget::PopDesktopWidget as RustWidget;


#[no_mangle]
pub struct PopDesktopWidget;

#[no_mangle]
pub extern "C" fn pop_desktop_widget_new(
    stack: *mut gtk_sys::GtkStack
) -> *mut PopDesktopWidget {
    unsafe {
        gtk::set_initialized();
    }

    Box::into_raw(Box::new(RustWidget::new(
        unsafe { &gtk::Stack::from_glib_none(stack) }
    ))) as *mut PopDesktopWidget
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
pub extern "C" fn pop_desktop_widget_gis_panel_page(header: *mut GtkWidget) -> *mut GtkWidget {
    unsafe {
        gtk::set_initialized();
    }

    let header = unsafe { gtk::Widget::from_glib_none(header) };
    pop_desktop_widget::gis::panel::page(&header).to_glib_full()
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_gresource_init() {
    pop_desktop_widget::gresource::init().expect("failed to load gresource for pop-desktop-widget");
}