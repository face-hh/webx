use std::cell::RefCell;

use glib::Properties;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::HistoryItem;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::HistoryObject)]
pub struct HistoryObject {
    #[property(name = "url", get, set, type = String, member = url)]
    #[property(name = "position", get, set, type = i32, member = position)]
    #[property(name = "date", get, set, type = String, member = date)]
    history: RefCell<HistoryItem>
}

#[glib::object_subclass]
impl ObjectSubclass for HistoryObject {
    const NAME: &'static str = "GtkNaptureHistory";
    type Type = super::HistoryObject;
}

#[glib::derived_properties]
impl ObjectImpl for HistoryObject {}
