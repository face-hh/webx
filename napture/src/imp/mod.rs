use gio::Settings;
use gtk::subclass::prelude::*;
use gtk::{gio, ApplicationWindow};
use std::cell::OnceCell;

// ANCHOR: imp
#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "BussinNapture";
    type Type = super::Window;
    type ParentType = ApplicationWindow;
}
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

    }
}
impl WidgetImpl for Window {}
impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self) -> glib::Propagation {

        glib::Propagation::Proceed
    }
}
impl ApplicationWindowImpl for Window {}
