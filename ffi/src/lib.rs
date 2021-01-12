use glib::object::{Cast, ObjectType};
use pop_desktop_widget::PopDesktopWidget as RustWidget;
use std::ptr;

#[no_mangle]
pub struct PopDesktopWidget;

#[no_mangle]
pub extern "C" fn pop_desktop_widget_new() -> *mut PopDesktopWidget {
    unsafe {
        gtk::set_initialized();
    }

    Box::into_raw(Box::new(RustWidget::new())) as *mut PopDesktopWidget
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_grab_focus(ptr: *const PopDesktopWidget) {
    if let Some(rust_widget) = unsafe { (ptr as *const RustWidget).as_ref() } {
        rust_widget.grab_focus();
    }
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_widget(
    ptr: *const PopDesktopWidget,
) -> *mut gtk_sys::GtkWidget {
    let value = unsafe { (ptr as *const RustWidget).as_ref() };
    value.map_or(ptr::null_mut(), |widget| {
        let widget: &gtk::Container = widget.as_ref();
        widget.upcast_ref::<gtk::Widget>().as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn pop_desktop_widget_free(widget: *mut PopDesktopWidget) {
    unsafe { Box::from_raw(widget as *mut RustWidget) };
}
