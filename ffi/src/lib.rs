use glib::translate::FromGlibPtrNone;
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
