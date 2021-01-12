#[macro_use]
extern crate gtk_extras;

use gtk::prelude::*;
use libhandy::prelude::*;

use std::cell::Cell;
use std::rc::Rc;
use std::ops::Deref;

pub struct PopDesktopWidget(gtk::Container);

fn switch_row<C: ContainerExt>(container: &C, title: Option<&str>) -> gtk::Switch {
    let switch = cascade! {
        gtk::Switch::new();
        ..set_valign(gtk::Align::Center);
    };
    let row = cascade! {
        libhandy::ActionRow::new();
        ..set_title(title);
        ..add(&switch);
    };
    container.add(&row);
    switch
}

fn top_bar<C: ContainerExt>(container: &C) {
    let list_box = cascade! {
        gtk::ListBox::new();
        ..set_selection_mode(gtk::SelectionMode::None);
    };
    container.add(&list_box);

    switch_row(&list_box, Some("Show Workspaces Button"));
    switch_row(&list_box, Some("Show Applications Button"));
}

impl PopDesktopWidget {
    pub fn new() -> Self {

        //TODO: settings
        top_bar(&container);

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
